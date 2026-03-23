use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use bytes::Bytes;
use reqwest::header::HeaderMap;
use tracing::{debug, warn};

use crate::error::HttpError;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_BACKOFF_MS: [u64; 3] = [1000, 2000, 4000];

/// Configuration for building an [`HttpClient`].
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout_secs: u64,
    pub max_retries: u32,
    /// Backoff durations per attempt in milliseconds. Must have at least `max_retries` entries.
    pub backoff_ms: Vec<u64>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            max_retries: DEFAULT_MAX_RETRIES,
            backoff_ms: DEFAULT_BACKOFF_MS.to_vec(),
        }
    }
}

/// Raw HTTP response returned by [`HttpClient::send`].
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Bytes,
    pub rate_limit_remaining: Option<u32>,
    pub endpoint: String,
}

/// Async HTTP client with retry, rate-limit handling, and conditional-request caching.
#[derive(Debug)]
pub struct HttpClient {
    inner: reqwest::Client,
    max_retries: u32,
    backoff_ms: Vec<u64>,
    cache: Mutex<HashMap<String, CacheEntry>>,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    etag: Option<String>,
    last_modified: Option<String>,
    body: Bytes,
    status: u16,
}

impl HttpClient {
    /// Create a new client with the given configuration.
    ///
    /// `max_retries` is clamped to a minimum of 1 (at least one attempt is always made).
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(concat!("alpha-gov-api/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| HttpError::BuildClient { source: e })?;

        Ok(Self {
            inner: client,
            max_retries: config.max_retries.max(1),
            backoff_ms: config.backoff_ms,
            cache: Mutex::new(HashMap::new()),
        })
    }

    /// Send a request with automatic retry, rate-limit handling, and ETag caching.
    ///
    /// The caller builds a [`reqwest::Request`] (via [`HttpClient::inner`]) and passes it here.
    /// Transient errors (429, 5xx, timeouts) are retried with exponential backoff.
    pub async fn send(&self, request: reqwest::Request) -> crate::Result<HttpResponse> {
        let url = request.url().to_string();
        let method = request.method().clone();

        debug!(%url, %method, "sending request");

        // Attach conditional headers if we have a cache entry for this URL.
        let request = self.attach_cache_headers(request);

        for attempt in 0..self.max_retries {
            let req = match request.try_clone() {
                Some(r) => r,
                None => {
                    // Streaming bodies cannot be cloned; send once without retry.
                    return self.execute_once(request, &url).await;
                }
            };

            match self.inner.execute(req).await {
                Err(e) if e.is_timeout() => {
                    warn!(%url, attempt, "request timed out");
                    if attempt + 1 >= self.max_retries {
                        return Err(HttpError::Timeout { url }.into());
                    }
                    self.backoff(attempt).await;
                }
                Err(e) if e.is_connect() || e.is_request() => {
                    warn!(%url, attempt, error = %e, "network error");
                    if attempt + 1 >= self.max_retries {
                        return Err(HttpError::Network { url, source: e }.into());
                    }
                    self.backoff(attempt).await;
                }
                Err(e) => {
                    return Err(HttpError::Network { url, source: e }.into());
                }
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let headers = resp.headers().clone();
                    let rate_limit_remaining = parse_rate_limit_remaining(&headers);

                    // 304 Not Modified — return cached body.
                    if status == 304 {
                        debug!(%url, "cache hit (304)");
                        return match self.cache_get(&url) {
                            Some(entry) => Ok(HttpResponse {
                                status: entry.status,
                                body: entry.body,
                                rate_limit_remaining,
                                endpoint: url,
                            }),
                            None => Err(HttpError::Api {
                                status: 304,
                                url,
                                body: "304 received but no cached response available".into(),
                            }
                            .into()),
                        };
                    }

                    // 429 Too Many Requests — sleep per Retry-After, then retry.
                    if status == 429 {
                        let sleep_ms = parse_retry_after(&headers)
                            .map(|s| s * 1000)
                            .unwrap_or(self.backoff_duration_ms(attempt));
                        warn!(%url, attempt, sleep_ms, "rate limited (429)");
                        if attempt + 1 >= self.max_retries {
                            return Err(HttpError::MaxRetriesExceeded {
                                url,
                                attempts: self.max_retries,
                            }
                            .into());
                        }
                        tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
                        continue;
                    }

                    // Transient server error — retry with backoff.
                    if status >= 500 {
                        warn!(%url, status, attempt, "transient server error");
                        if attempt + 1 >= self.max_retries {
                            let body = resp.text().await.unwrap_or_default();
                            return Err(HttpError::Api { status, url, body }.into());
                        }
                        self.backoff(attempt).await;
                        continue;
                    }

                    // Auth error — never retry.
                    if status == 401 || status == 403 {
                        return Err(HttpError::Auth { status, url }.into());
                    }

                    // Read body.
                    let body = resp.bytes().await.map_err(|e| HttpError::Network {
                        url: url.clone(),
                        source: e,
                    })?;

                    // Non-transient client error.
                    if status >= 400 {
                        return Err(HttpError::Api {
                            status,
                            url,
                            body: String::from_utf8_lossy(&body).into_owned(),
                        }
                        .into());
                    }

                    // Success — update cache if the response includes cache headers.
                    let etag = header_str(&headers, "etag");
                    let last_modified = header_str(&headers, "last-modified");
                    if etag.is_some() || last_modified.is_some() {
                        self.cache_insert(
                            url.clone(),
                            CacheEntry {
                                etag,
                                last_modified,
                                body: body.clone(),
                                status,
                            },
                        );
                    }

                    return Ok(HttpResponse {
                        status,
                        body,
                        rate_limit_remaining,
                        endpoint: url,
                    });
                }
            }
        }

        Err(HttpError::MaxRetriesExceeded {
            url,
            attempts: self.max_retries,
        }
        .into())
    }

    /// Access the inner `reqwest::Client` for building requests.
    pub fn inner(&self) -> &reqwest::Client {
        &self.inner
    }

    fn attach_cache_headers(&self, mut request: reqwest::Request) -> reqwest::Request {
        let url = request.url().to_string();
        if let Some(entry) = self.cache_get(&url) {
            if let Some(etag) = &entry.etag
                && let Ok(val) = etag.parse()
            {
                request.headers_mut().insert("if-none-match", val);
            }
            if let Some(lm) = &entry.last_modified
                && let Ok(val) = lm.parse()
            {
                request.headers_mut().insert("if-modified-since", val);
            }
        }
        request
    }

    /// Execute a single request without retry (for non-cloneable streaming bodies).
    async fn execute_once(
        &self,
        request: reqwest::Request,
        url: &str,
    ) -> crate::Result<HttpResponse> {
        let resp = self
            .inner
            .execute(request)
            .await
            .map_err(|e| HttpError::Network {
                url: url.to_string(),
                source: e,
            })?;

        let status = resp.status().as_u16();
        let rate_limit_remaining = parse_rate_limit_remaining(resp.headers());

        // 304 is not meaningful without cache support (streaming bodies skip caching).
        if status == 304 {
            return Err(HttpError::Api {
                status: 304,
                url: url.to_string(),
                body: "304 received but caching not available for streaming requests".into(),
            }
            .into());
        }

        if status == 401 || status == 403 {
            return Err(HttpError::Auth {
                status,
                url: url.to_string(),
            }
            .into());
        }

        let body = resp.bytes().await.map_err(|e| HttpError::Network {
            url: url.to_string(),
            source: e,
        })?;

        if status >= 400 {
            return Err(HttpError::Api {
                status,
                url: url.to_string(),
                body: String::from_utf8_lossy(&body).into_owned(),
            }
            .into());
        }

        Ok(HttpResponse {
            status,
            body,
            rate_limit_remaining,
            endpoint: url.to_string(),
        })
    }

    fn backoff_duration_ms(&self, attempt: u32) -> u64 {
        self.backoff_ms
            .get(attempt as usize)
            .copied()
            .unwrap_or(4000)
    }

    async fn backoff(&self, attempt: u32) {
        let ms = self.backoff_duration_ms(attempt);
        debug!(ms, attempt, "backing off");
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }

    fn cache_get(&self, url: &str) -> Option<CacheEntry> {
        self.cache.lock().unwrap().get(url).cloned()
    }

    fn cache_insert(&self, url: String, entry: CacheEntry) {
        self.cache.lock().unwrap().insert(url, entry);
    }
}

fn parse_rate_limit_remaining(headers: &HeaderMap) -> Option<u32> {
    headers
        .get("x-ratelimit-remaining")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

fn parse_retry_after(headers: &HeaderMap) -> Option<u64> {
    let value = headers.get("retry-after")?.to_str().ok()?;
    // Try integer seconds first.
    if let Ok(secs) = value.parse::<u64>() {
        return Some(secs);
    }
    // Try HTTP-date (RFC 2822).
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(value) {
        let now = chrono::Utc::now();
        let delta = dt.signed_duration_since(now).num_seconds().max(0) as u64;
        return Some(delta);
    }
    None
}

fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Build a test client with zero-duration backoffs for fast tests.
    fn fast_client() -> HttpClient {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        HttpClient {
            inner: client,
            max_retries: DEFAULT_MAX_RETRIES,
            backoff_ms: vec![0, 0, 0],
            cache: Mutex::new(HashMap::new()),
        }
    }

    fn get_request(server: &MockServer, path_str: &str) -> reqwest::Request {
        reqwest::Client::new()
            .get(format!("{}{}", server.uri(), path_str))
            .build()
            .unwrap()
    }

    #[test]
    fn build_client_default() {
        let client = HttpClient::new(HttpClientConfig::default());
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn success_returns_body() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"{\"name\":\"test\"}" as &[u8])
                    .insert_header("x-ratelimit-remaining", "99"),
            )
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/test");
        let resp = client.send(req).await.unwrap();

        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Bytes::from_static(b"{\"name\":\"test\"}"));
        assert_eq!(resp.rate_limit_remaining, Some(99));
    }

    #[tokio::test]
    async fn no_retry_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .expect(1)
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/missing");
        let result = client.send(req).await;

        let err = result.unwrap_err();
        match *err.downcast_http().unwrap() {
            HttpError::Api {
                status, ref body, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(body, "not found");
            }
            _ => panic!("expected HttpError::Api"),
        }
    }

    #[tokio::test]
    async fn no_retry_on_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/secret"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1)
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/secret");
        let result = client.send(req).await;

        let err = result.unwrap_err();
        match *err.downcast_http().unwrap() {
            HttpError::Auth { status, .. } => assert_eq!(status, 401),
            _ => panic!("expected HttpError::Auth"),
        }
    }

    #[tokio::test]
    async fn retry_on_503() {
        let server = MockServer::start().await;

        // First two calls return 503, third returns 200.
        Mock::given(method("GET"))
            .and(path("/flaky"))
            .respond_with(ResponseTemplate::new(503))
            .up_to_n_times(2)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/flaky"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok" as &[u8]))
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/flaky");
        let resp = client.send(req).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Bytes::from_static(b"ok"));
    }

    #[tokio::test]
    async fn api_error_on_persistent_500() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/down"))
            .respond_with(ResponseTemplate::new(500).set_body_string("error"))
            .expect(3)
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/down");
        let result = client.send(req).await;

        let err = result.unwrap_err();
        match *err.downcast_http().unwrap() {
            HttpError::Api { status, .. } => assert_eq!(status, 500),
            _ => panic!("expected HttpError::Api for final 500"),
        }
    }

    #[tokio::test]
    async fn max_retries_exceeded_on_persistent_429() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/throttled"))
            .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
            .expect(3)
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/throttled");
        let result = client.send(req).await;

        let err = result.unwrap_err();
        match *err.downcast_http().unwrap() {
            HttpError::MaxRetriesExceeded { attempts, .. } => assert_eq!(attempts, 3),
            _ => panic!("expected HttpError::MaxRetriesExceeded"),
        }
    }

    #[tokio::test]
    async fn etag_cache_returns_cached_body_on_304() {
        let server = MockServer::start().await;

        // First request: 200 with ETag.
        Mock::given(method("GET"))
            .and(path("/cached"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"{\"data\":\"cached\"}" as &[u8])
                    .insert_header("etag", "\"abc123\""),
            )
            .up_to_n_times(1)
            .mount(&server)
            .await;

        let client = fast_client();

        // First call — populates cache.
        let req = get_request(&server, "/cached");
        let resp = client.send(req).await.unwrap();
        assert_eq!(resp.status, 200);

        // Second request: server returns 304.
        Mock::given(method("GET"))
            .and(path("/cached"))
            .respond_with(ResponseTemplate::new(304))
            .mount(&server)
            .await;

        let req = get_request(&server, "/cached");
        let resp = client.send(req).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Bytes::from_static(b"{\"data\":\"cached\"}"));
    }

    #[tokio::test]
    async fn rate_limit_429_with_retry_after() {
        let server = MockServer::start().await;

        // First call: 429 with Retry-After: 0 (instant for tests).
        Mock::given(method("GET"))
            .and(path("/limited"))
            .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        // Second call: 200.
        Mock::given(method("GET"))
            .and(path("/limited"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok" as &[u8]))
            .mount(&server)
            .await;

        let client = fast_client();
        let req = get_request(&server, "/limited");
        let resp = client.send(req).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn parse_rate_limit_remaining_valid() {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-remaining", "42".parse().unwrap());
        assert_eq!(parse_rate_limit_remaining(&headers), Some(42));
    }

    #[test]
    fn parse_rate_limit_remaining_missing() {
        let headers = HeaderMap::new();
        assert_eq!(parse_rate_limit_remaining(&headers), None);
    }

    #[test]
    fn parse_retry_after_integer() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", "30".parse().unwrap());
        assert_eq!(parse_retry_after(&headers), Some(30));
    }

    #[test]
    fn parse_retry_after_missing() {
        let headers = HeaderMap::new();
        assert_eq!(parse_retry_after(&headers), None);
    }
}
