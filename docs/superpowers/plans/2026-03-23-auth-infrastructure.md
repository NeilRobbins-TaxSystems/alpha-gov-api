# Auth Infrastructure Implementation Plan (Issue #4)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add authentication flows for all API providers — API key (CH), OAuth client credentials (HMRC application-restricted), OAuth authorization code with local callback (HMRC user-restricted), and token caching with refresh.

**Architecture:** New `auth` module in the core crate with four files: `api_key.rs` (Basic auth header), `oauth.rs` (token exchange/refresh), `token_store.rs` (in-memory cache with TTL), `callback.rs` (TCP callback server for auth code flow). A top-level `authenticate()` function dispatches to the correct flow based on an `AuthMethod` enum. Token exchange calls use bare `reqwest::Client` (not `HttpClient`) to avoid error-domain conflicts.

**Tech Stack:** `base64` (Basic encoding), `rand` (CSRF state), `open` (browser launch), `tokio::net` (callback TCP server), `wiremock` (tests)

**Spec:** `docs/superpowers/specs/2026-03-23-auth-infrastructure-design.md`

---

## File Structure

**Create:**
- `crates/alpha-gov-api-core/src/auth/mod.rs` — `AuthMethod` enum, `authenticate()` entry point, `bearer_header()`, `generate_state()`, re-exports
- `crates/alpha-gov-api-core/src/auth/api_key.rs` — `basic_auth_header()` for CH API key → HTTP Basic
- `crates/alpha-gov-api-core/src/auth/oauth.rs` — `exchange_client_credentials()`, `exchange_authorization_code()`, `refresh_access_token()`, shared `post_token_request()`, `TokenResponse`
- `crates/alpha-gov-api-core/src/auth/token_store.rs` — `TokenStore` with `get_valid_token()`, `get_refresh_token()`, `store_token()`, `clear()`
- `crates/alpha-gov-api-core/src/auth/callback.rs` — `wait_for_callback()` TCP server loop

**Modify:**
- `Cargo.toml` — add `base64`, `rand`, `open` to `[workspace.dependencies]`; add `net`, `io-util` to tokio features
- `crates/alpha-gov-api-core/Cargo.toml` — add `base64`, `rand`, `open` to `[dependencies]`
- `crates/alpha-gov-api-core/src/error.rs` — add `AuthError` enum, `AppError::Auth` variant, `downcast_auth()`, `From<AuthError>`
- `crates/alpha-gov-api-core/src/config.rs:13-20` — add `hmrc.refresh_token` to `CREDENTIAL_KEYS`
- `crates/alpha-gov-api-core/src/lib.rs` — add `pub mod auth` and re-exports
- `CLAUDE.md` — document auth module

---

### Task 1: Foundation — Dependencies, AuthError, config, module skeleton

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/alpha-gov-api-core/Cargo.toml`
- Modify: `crates/alpha-gov-api-core/src/error.rs`
- Modify: `crates/alpha-gov-api-core/src/config.rs`
- Modify: `crates/alpha-gov-api-core/src/lib.rs`
- Create: `crates/alpha-gov-api-core/src/auth/mod.rs`

- [ ] **Step 1: Add workspace dependencies**

In `Cargo.toml`, add to `[workspace.dependencies]`:
```toml
base64 = "0.22"
open = "5"
rand = "0.9"
```

And update tokio to include `net` and `io-util`:
```toml
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time", "net", "io-util"] }
```

- [ ] **Step 2: Add dependencies to core crate**

In `crates/alpha-gov-api-core/Cargo.toml`, add to `[dependencies]`:
```toml
base64 = { workspace = true }
open = { workspace = true }
rand = { workspace = true }
```

- [ ] **Step 3: Add AuthError and AppError::Auth to error.rs**

In `crates/alpha-gov-api-core/src/error.rs`, add the `AuthError` enum after `HttpError`:

```rust
/// Authentication and token errors.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing credential: {key}")]
    MissingCredential { key: String },

    #[error("token exchange failed ({status}): {body}")]
    TokenExchangeFailed { status: u16, body: String },

    #[error("token refresh failed ({status}): {body}")]
    TokenRefreshFailed { status: u16, body: String },

    #[error("callback server error: {detail}")]
    CallbackServerFailed { detail: String },

    #[error("OAuth state mismatch — possible CSRF attack")]
    InvalidState,

    #[error("failed to open browser: {source}")]
    BrowserOpenFailed { source: std::io::Error },

    #[error("invalid token response: {detail}")]
    InvalidTokenResponse { detail: String },
}
```

Add variant to `AppError`:
```rust
#[error(transparent)]
Auth(Box<AuthError>),
```

Add `downcast_auth()` to `impl AppError`:
```rust
pub fn downcast_auth(self) -> Option<Box<AuthError>> {
    match self {
        AppError::Auth(e) => Some(e),
        _ => None,
    }
}
```

Add `From<AuthError>`:
```rust
impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        AppError::Auth(Box::new(e))
    }
}
```

- [ ] **Step 4: Add hmrc.refresh_token to CREDENTIAL_KEYS**

In `crates/alpha-gov-api-core/src/config.rs`, add to `CREDENTIAL_KEYS` array:
```rust
("hmrc.refresh_token", "ALPHA_GOV_API_HMRC_REFRESH_TOKEN"),
```

- [ ] **Step 5: Create empty auth module skeleton**

Create `crates/alpha-gov-api-core/src/auth/mod.rs`:
```rust
pub mod api_key;
pub mod callback;
pub mod oauth;
pub mod token_store;
```

In `crates/alpha-gov-api-core/src/lib.rs`, add:
```rust
pub mod auth;
```

And add to the re-exports:
```rust
pub use error::{AppError, AuthError, ConfigError, HttpError, Result};
```

- [ ] **Step 6: Create stub files so the module compiles**

Create empty files (all `crates/alpha-gov-api-core/src/auth/`):
- `api_key.rs` — empty
- `oauth.rs` — empty
- `token_store.rs` — empty
- `callback.rs` — empty

- [ ] **Step 7: Verify compilation**

Run: `rustup run stable cargo build`
Expected: compiles with no errors (may have unused warnings, that's fine)

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml Cargo.lock crates/alpha-gov-api-core/Cargo.toml \
  crates/alpha-gov-api-core/src/error.rs \
  crates/alpha-gov-api-core/src/config.rs \
  crates/alpha-gov-api-core/src/lib.rs \
  crates/alpha-gov-api-core/src/auth/
git commit -m "Add auth module foundation: AuthError, dependencies, module skeleton (issue #4)"
```

---

### Task 2: TokenStore

**Files:**
- Create: `crates/alpha-gov-api-core/src/auth/token_store.rs`

- [ ] **Step 1: Write failing tests**

In `crates/alpha-gov-api-core/src/auth/token_store.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_retrieve_valid_token() {
        let store = TokenStore::new();
        store.store_token("hmrc", "access-123".into(), Some(3600), None);
        assert_eq!(store.get_valid_token("hmrc"), Some("access-123".into()));
    }

    #[test]
    fn returns_none_for_unknown_key() {
        let store = TokenStore::new();
        assert_eq!(store.get_valid_token("unknown"), None);
    }

    #[test]
    fn returns_none_when_expired() {
        let store = TokenStore::new();
        // expires_in: 0 means already expired
        store.store_token("hmrc", "expired-tok".into(), Some(0), None);
        assert_eq!(store.get_valid_token("hmrc"), None);
    }

    #[test]
    fn respects_expiry_buffer() {
        let store = TokenStore::new();
        // 29 seconds left, but 30s buffer means it's treated as expired
        store.store_token("hmrc", "almost-expired".into(), Some(29), None);
        assert_eq!(store.get_valid_token("hmrc"), None);

        // 31 seconds left — safely beyond the 30s buffer
        store.store_token("hmrc", "still-valid".into(), Some(31), None);
        assert_eq!(store.get_valid_token("hmrc"), Some("still-valid".into()));
    }

    #[test]
    fn token_without_expiry_is_always_valid() {
        let store = TokenStore::new();
        store.store_token("hmrc", "no-expiry".into(), None, None);
        assert_eq!(store.get_valid_token("hmrc"), Some("no-expiry".into()));
    }

    #[test]
    fn overwrite_existing_token() {
        let store = TokenStore::new();
        store.store_token("hmrc", "old".into(), Some(3600), None);
        store.store_token("hmrc", "new".into(), Some(3600), None);
        assert_eq!(store.get_valid_token("hmrc"), Some("new".into()));
    }

    #[test]
    fn get_refresh_token() {
        let store = TokenStore::new();
        store.store_token("hmrc", "access".into(), Some(3600), Some("refresh-tok".into()));
        assert_eq!(store.get_refresh_token("hmrc"), Some("refresh-tok".into()));
    }

    #[test]
    fn get_refresh_token_when_none() {
        let store = TokenStore::new();
        store.store_token("hmrc", "access".into(), Some(3600), None);
        assert_eq!(store.get_refresh_token("hmrc"), None);
    }

    #[test]
    fn clear_removes_token() {
        let store = TokenStore::new();
        store.store_token("hmrc", "access".into(), Some(3600), None);
        store.clear("hmrc");
        assert_eq!(store.get_valid_token("hmrc"), None);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `rustup run stable cargo test -p alpha-gov-api-core token_store`
Expected: FAIL — `TokenStore` not defined

- [ ] **Step 3: Implement TokenStore**

Add implementation above the tests in `token_store.rs`:

```rust
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const EXPIRY_BUFFER_SECS: u64 = 30;

/// Thread-safe in-memory token cache with TTL.
pub struct TokenStore {
    tokens: Mutex<HashMap<String, TokenEntry>>,
}

struct TokenEntry {
    access_token: String,
    expires_at: Option<Instant>,
    refresh_token: Option<String>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the access token if it exists and is not expired (with 30s buffer).
    pub fn get_valid_token(&self, key: &str) -> Option<String> {
        let tokens = self.tokens.lock().unwrap();
        let entry = tokens.get(key)?;
        if let Some(expires_at) = entry.expires_at {
            if Instant::now() + Duration::from_secs(EXPIRY_BUFFER_SECS) >= expires_at {
                return None;
            }
        }
        Some(entry.access_token.clone())
    }

    /// Returns the refresh token if stored.
    pub fn get_refresh_token(&self, key: &str) -> Option<String> {
        let tokens = self.tokens.lock().unwrap();
        tokens.get(key)?.refresh_token.clone()
    }

    /// Cache a token with optional expiry and refresh token.
    pub fn store_token(
        &self,
        key: &str,
        access_token: String,
        expires_in_secs: Option<u64>,
        refresh_token: Option<String>,
    ) {
        let expires_at = expires_in_secs.map(|s| Instant::now() + Duration::from_secs(s));
        self.tokens.lock().unwrap().insert(
            key.to_string(),
            TokenEntry {
                access_token,
                expires_at,
                refresh_token,
            },
        );
    }

    /// Remove a token entry.
    pub fn clear(&self, key: &str) {
        self.tokens.lock().unwrap().remove(key);
    }
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `rustup run stable cargo test -p alpha-gov-api-core token_store`
Expected: all 9 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/alpha-gov-api-core/src/auth/token_store.rs
git commit -m "Add TokenStore with TTL-based token caching (issue #4)"
```

---

### Task 3: API key auth

**Files:**
- Create: `crates/alpha-gov-api-core/src/auth/api_key.rs`

- [ ] **Step 1: Write failing test**

In `crates/alpha-gov-api-core/src/auth/api_key.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::AUTHORIZATION;

    #[test]
    fn basic_auth_header_encodes_correctly() {
        let headers = basic_auth_header("test-api-key-123");
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        // base64("test-api-key-123:") = "dGVzdC1hcGkta2V5LTEyMzo="
        assert_eq!(auth, "Basic dGVzdC1hcGkta2V5LTEyMzo=");
    }

    #[test]
    fn basic_auth_header_handles_empty_key() {
        let headers = basic_auth_header("");
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        // base64(":") = "Og=="
        assert_eq!(auth, "Basic Og==");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `rustup run stable cargo test -p alpha-gov-api-core api_key`
Expected: FAIL — `basic_auth_header` not defined

- [ ] **Step 3: Implement basic_auth_header**

Add above the tests:

```rust
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

/// Build an `Authorization: Basic` header from a Companies House API key.
///
/// The key is sent as the HTTP Basic username with an empty password.
pub fn basic_auth_header(api_key: &str) -> HeaderMap {
    let encoded = STANDARD.encode(format!("{api_key}:"));
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {encoded}")).expect("valid header value"),
    );
    headers
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `rustup run stable cargo test -p alpha-gov-api-core api_key`
Expected: 2 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/alpha-gov-api-core/src/auth/api_key.rs
git commit -m "Add API key auth with HTTP Basic encoding (issue #4)"
```

---

### Task 4: OAuth token exchange helpers

**Files:**
- Create: `crates/alpha-gov-api-core/src/auth/oauth.rs`

- [ ] **Step 1: Write failing tests**

In `crates/alpha-gov-api-core/src/auth/oauth.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_string_contains, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn client_credentials_exchange_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .and(body_string_contains("grant_type=client_credentials"))
            .and(body_string_contains("client_id=my-id"))
            .and(body_string_contains("client_secret=my-secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "tok-123",
                "expires_in": 3600,
                "token_type": "bearer"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let resp = exchange_client_credentials(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "my-id",
            "my-secret",
            &[],
        )
        .await
        .unwrap();

        assert_eq!(resp.access_token, "tok-123");
        assert_eq!(resp.expires_in, Some(3600));
    }

    #[tokio::test]
    async fn client_credentials_exchange_with_scopes() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .and(body_string_contains("scope=read%3Avat+write%3Avat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "scoped-tok",
                "expires_in": 1800,
                "token_type": "bearer"
            })))
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let resp = exchange_client_credentials(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "id",
            "secret",
            &["read:vat".into(), "write:vat".into()],
        )
        .await
        .unwrap();

        assert_eq!(resp.access_token, "scoped-tok");
    }

    #[tokio::test]
    async fn client_credentials_exchange_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(
                ResponseTemplate::new(401).set_body_string("invalid_client"),
            )
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let err = exchange_client_credentials(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "bad-id",
            "bad-secret",
            &[],
        )
        .await
        .unwrap_err();

        match err {
            AuthError::TokenExchangeFailed { status, body } => {
                assert_eq!(status, 401);
                assert_eq!(body, "invalid_client");
            }
            _ => panic!("expected TokenExchangeFailed, got {err:?}"),
        }
    }

    #[tokio::test]
    async fn authorization_code_exchange_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .and(body_string_contains("grant_type=authorization_code"))
            .and(body_string_contains("code=auth-code-xyz"))
            .and(body_string_contains("redirect_uri=http%3A%2F%2Flocalhost%3A9004%2Fcallback"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "user-tok",
                "expires_in": 14400,
                "refresh_token": "refresh-tok",
                "token_type": "bearer"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let resp = exchange_authorization_code(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "id",
            "secret",
            "auth-code-xyz",
            "http://localhost:9004/callback",
        )
        .await
        .unwrap();

        assert_eq!(resp.access_token, "user-tok");
        assert_eq!(resp.refresh_token, Some("refresh-tok".into()));
        assert_eq!(resp.expires_in, Some(14400));
    }

    #[tokio::test]
    async fn refresh_token_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .and(body_string_contains("refresh_token=rt-old"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "new-access-tok",
                "expires_in": 14400,
                "refresh_token": "rt-new",
                "token_type": "bearer"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let resp = refresh_access_token(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "id",
            "secret",
            "rt-old",
        )
        .await
        .unwrap();

        assert_eq!(resp.access_token, "new-access-tok");
        assert_eq!(resp.refresh_token, Some("rt-new".into()));
    }

    #[tokio::test]
    async fn refresh_failure_returns_refresh_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(
                ResponseTemplate::new(400).set_body_string("invalid_grant"),
            )
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let err = refresh_access_token(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "id",
            "secret",
            "bad-rt",
        )
        .await
        .unwrap_err();

        match err {
            AuthError::TokenRefreshFailed { status, body } => {
                assert_eq!(status, 400);
                assert_eq!(body, "invalid_grant");
            }
            _ => panic!("expected TokenRefreshFailed, got {err:?}"),
        }
    }

    #[tokio::test]
    async fn invalid_json_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string("not json"),
            )
            .mount(&server)
            .await;

        let client = reqwest::Client::new();
        let err = exchange_client_credentials(
            &client,
            &format!("{}/oauth/token", server.uri()),
            "id",
            "secret",
            &[],
        )
        .await
        .unwrap_err();

        assert!(matches!(err, AuthError::InvalidTokenResponse { .. }));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `rustup run stable cargo test -p alpha-gov-api-core oauth`
Expected: FAIL — functions not defined

- [ ] **Step 3: Implement OAuth token exchange functions**

Add above the tests in `oauth.rs`:

```rust
use reqwest::Client;
use serde::Deserialize;

use crate::error::AuthError;

/// Token response from an OAuth 2.0 token endpoint.
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
}

/// Exchange client credentials for an access token (HMRC application-restricted).
pub async fn exchange_client_credentials(
    client: &Client,
    token_url: &str,
    client_id: &str,
    client_secret: &str,
    scopes: &[String],
) -> Result<TokenResponse, AuthError> {
    let mut params: Vec<(&str, &str)> = vec![
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    let scope_str = scopes.join(" ");
    if !scopes.is_empty() {
        params.push(("scope", &scope_str));
    }
    post_token_request(client, token_url, &params).await
}

/// Exchange an authorization code for tokens (HMRC user-restricted).
pub async fn exchange_authorization_code(
    client: &Client,
    token_url: &str,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<TokenResponse, AuthError> {
    let params: Vec<(&str, &str)> = vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    post_token_request(client, token_url, &params).await
}

/// Refresh an access token using a refresh token.
pub async fn refresh_access_token(
    client: &Client,
    token_url: &str,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenResponse, AuthError> {
    let params: Vec<(&str, &str)> = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    match post_token_request(client, token_url, &params).await {
        Ok(resp) => Ok(resp),
        Err(AuthError::TokenExchangeFailed { status, body }) => {
            Err(AuthError::TokenRefreshFailed { status, body })
        }
        Err(e) => Err(e),
    }
}

async fn post_token_request(
    client: &Client,
    token_url: &str,
    params: &[(&str, &str)],
) -> Result<TokenResponse, AuthError> {
    let resp = client
        .post(token_url)
        .form(params)
        .send()
        .await
        .map_err(|e| AuthError::TokenExchangeFailed {
            status: 0,
            body: e.to_string(),
        })?;

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AuthError::TokenExchangeFailed { status, body });
    }

    resp.json().await.map_err(|e| AuthError::InvalidTokenResponse {
        detail: e.to_string(),
    })
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `rustup run stable cargo test -p alpha-gov-api-core oauth`
Expected: all 7 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/alpha-gov-api-core/src/auth/oauth.rs
git commit -m "Add OAuth token exchange helpers: client credentials, auth code, refresh (issue #4)"
```

---

### Task 5: Callback server

**Files:**
- Create: `crates/alpha-gov-api-core/src/auth/callback.rs`

- [ ] **Step 1: Write failing tests**

In `crates/alpha-gov-api-core/src/auth/callback.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn returns_code_on_valid_callback() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let state = "test-state-abc";

        let server = tokio::spawn(async move {
            wait_for_callback(listener, state, Duration::from_secs(5)).await
        });

        let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        let request = format!(
            "GET /callback?code=auth-code-456&state={state} HTTP/1.1\r\nHost: localhost\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).await.unwrap();

        let code = server.await.unwrap().unwrap();
        assert_eq!(code, "auth-code-456");
    }

    #[tokio::test]
    async fn ignores_non_callback_paths() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let state = "test-state-def";

        let server = tokio::spawn(async move {
            wait_for_callback(listener, state, Duration::from_secs(5)).await
        });

        // Send favicon request first — should be ignored
        let mut stream1 = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        stream1
            .write_all(b"GET /favicon.ico HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
        drop(stream1);

        // Small delay to ensure first request is processed
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Then send the real callback
        let mut stream2 = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        let request = format!(
            "GET /callback?code=real-code&state={state} HTTP/1.1\r\nHost: localhost\r\n\r\n"
        );
        stream2.write_all(request.as_bytes()).await.unwrap();

        let code = server.await.unwrap().unwrap();
        assert_eq!(code, "real-code");
    }

    #[tokio::test]
    async fn rejects_state_mismatch() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = tokio::spawn(async move {
            wait_for_callback(listener, "expected-state", Duration::from_secs(5)).await
        });

        let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        stream
            .write_all(
                b"GET /callback?code=code&state=wrong-state HTTP/1.1\r\nHost: localhost\r\n\r\n",
            )
            .await
            .unwrap();

        let err = server.await.unwrap().unwrap_err();
        assert!(matches!(err, AuthError::InvalidState));
    }

    #[tokio::test]
    async fn times_out_when_no_callback() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();

        let err = wait_for_callback(listener, "state", Duration::from_millis(100))
            .await
            .unwrap_err();

        match err {
            AuthError::CallbackServerFailed { detail } => {
                assert!(detail.contains("timed out"), "got: {detail}");
            }
            _ => panic!("expected CallbackServerFailed, got {err:?}"),
        }
    }

    #[tokio::test]
    async fn handles_oauth_error_response() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = tokio::spawn(async move {
            wait_for_callback(listener, "state", Duration::from_secs(5)).await
        });

        let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        stream
            .write_all(
                b"GET /callback?error=access_denied&error_description=user+denied HTTP/1.1\r\nHost: localhost\r\n\r\n",
            )
            .await
            .unwrap();

        let err = server.await.unwrap().unwrap_err();
        match err {
            AuthError::CallbackServerFailed { detail } => {
                assert!(detail.contains("access_denied"), "got: {detail}");
            }
            _ => panic!("expected CallbackServerFailed, got {err:?}"),
        }
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `rustup run stable cargo test -p alpha-gov-api-core callback`
Expected: FAIL — `wait_for_callback` not defined

- [ ] **Step 3: Implement callback server**

Add above the tests in `callback.rs`:

```rust
use std::collections::HashMap;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::error::AuthError;

const SUCCESS_HTML: &str =
    "<html><body><h1>Authorization Successful</h1><p>You can close this tab.</p></body></html>";

/// Wait for an OAuth callback on the given listener.
///
/// Loops accepting TCP connections, ignoring non-callback requests (e.g. favicon).
/// Returns the authorization code when a valid callback with matching state arrives.
/// Returns an error on state mismatch, OAuth error response, or timeout.
pub async fn wait_for_callback(
    listener: TcpListener,
    expected_state: &str,
    timeout: Duration,
) -> Result<String, AuthError> {
    tokio::time::timeout(timeout, accept_callback_loop(listener, expected_state))
        .await
        .map_err(|_| AuthError::CallbackServerFailed {
            detail: format!(
                "timed out after {}s waiting for browser callback",
                timeout.as_secs()
            ),
        })?
}

async fn accept_callback_loop(
    listener: TcpListener,
    expected_state: &str,
) -> Result<String, AuthError> {
    loop {
        let (mut stream, _) =
            listener
                .accept()
                .await
                .map_err(|e| AuthError::CallbackServerFailed {
                    detail: e.to_string(),
                })?;

        let mut buf = vec![0u8; 4096];
        let n = stream
            .read(&mut buf)
            .await
            .map_err(|e| AuthError::CallbackServerFailed {
                detail: e.to_string(),
            })?;

        let request = String::from_utf8_lossy(&buf[..n]);
        let request_line = request.lines().next().unwrap_or("");

        // Parse "GET /path?query HTTP/1.1"
        let path = request_line.split_whitespace().nth(1).unwrap_or("");

        if !path.starts_with("/callback") {
            let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(resp.as_bytes()).await;
            continue;
        }

        let query = path.split('?').nth(1).unwrap_or("");
        let params: HashMap<&str, &str> = query
            .split('&')
            .filter_map(|p| p.split_once('='))
            .collect();

        // Check for OAuth error response
        if let Some(error) = params.get("error") {
            let desc = params.get("error_description").unwrap_or(&"unknown error");
            let html = format!(
                "<html><body><h1>Authorization Failed</h1><p>{desc}</p></body></html>"
            );
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{html}",
                html.len()
            );
            let _ = stream.write_all(resp.as_bytes()).await;
            return Err(AuthError::CallbackServerFailed {
                detail: format!("{error}: {desc}"),
            });
        }

        // No code param — might be a partial redirect, ignore
        let Some(code) = params.get("code") else {
            let resp = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(resp.as_bytes()).await;
            continue;
        };

        let Some(state) = params.get("state") else {
            let resp = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(resp.as_bytes()).await;
            return Err(AuthError::CallbackServerFailed {
                detail: "no state parameter in callback".into(),
            });
        };

        if *state != expected_state {
            let resp = "HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(resp.as_bytes()).await;
            return Err(AuthError::InvalidState);
        }

        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{SUCCESS_HTML}",
            SUCCESS_HTML.len()
        );
        let _ = stream.write_all(resp.as_bytes()).await;

        return Ok((*code).to_string());
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `rustup run stable cargo test -p alpha-gov-api-core callback`
Expected: all 5 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/alpha-gov-api-core/src/auth/callback.rs
git commit -m "Add OAuth callback server with timeout and favicon handling (issue #4)"
```

---

### Task 6: authenticate() entry point and module wiring

**Files:**
- Modify: `crates/alpha-gov-api-core/src/auth/mod.rs`
- Modify: `crates/alpha-gov-api-core/src/lib.rs`

- [ ] **Step 1: Write failing tests**

In `crates/alpha-gov-api-core/src/auth/mod.rs`, add tests at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::AUTHORIZATION;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn authenticate_api_key_returns_basic_header() {
        let method = AuthMethod::ApiKey(ApiKeyAuth {
            api_key: "test-key".into(),
        });
        let store = TokenStore::new();
        let client = reqwest::Client::new();
        let mut config =
            crate::AppConfig::load(Some(std::path::Path::new("/nonexistent.toml")), None).unwrap();

        let headers = authenticate(&method, &store, &client, &mut config)
            .await
            .unwrap();

        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert!(auth.starts_with("Basic "));
    }

    #[tokio::test]
    async fn authenticate_client_credentials_caches_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "cached-tok",
                "expires_in": 3600,
                "token_type": "bearer"
            })))
            .expect(1) // Only one request — second call hits cache
            .mount(&server)
            .await;

        let method = AuthMethod::ClientCredentials(ClientCredentialsAuth {
            client_id: "id".into(),
            client_secret: "secret".into(),
            token_url: format!("{}/oauth/token", server.uri()),
            scopes: vec![],
        });
        let store = TokenStore::new();
        let client = reqwest::Client::new();
        let mut config =
            crate::AppConfig::load(Some(std::path::Path::new("/nonexistent.toml")), None).unwrap();

        // First call — hits token endpoint
        let headers = authenticate(&method, &store, &client, &mut config)
            .await
            .unwrap();
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert_eq!(auth, "Bearer cached-tok");

        // Second call — cache hit, no second request
        let headers2 = authenticate(&method, &store, &client, &mut config)
            .await
            .unwrap();
        let auth2 = headers2.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert_eq!(auth2, "Bearer cached-tok");
    }

    #[test]
    fn generate_state_is_64_hex_chars() {
        let state = generate_state();
        assert_eq!(state.len(), 64);
        assert!(state.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn bearer_header_format() {
        let headers = bearer_header("my-token");
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert_eq!(auth, "Bearer my-token");
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `rustup run stable cargo test -p alpha-gov-api-core auth::tests`
Expected: FAIL — `authenticate`, `AuthMethod`, etc. not defined

- [ ] **Step 3: Implement mod.rs**

Replace `crates/alpha-gov-api-core/src/auth/mod.rs` with:

```rust
pub mod api_key;
pub mod callback;
pub mod oauth;
pub mod token_store;

pub use api_key::basic_auth_header;
pub use oauth::TokenResponse;
pub use token_store::TokenStore;

use std::time::Duration;

use rand::RngCore;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tracing::warn;

use crate::config::{get_credential, set_credential};
use crate::error::AuthError;
use crate::AppConfig;

/// Authentication methods supported by the tool.
pub enum AuthMethod {
    /// API key sent as HTTP Basic username with empty password (Companies House).
    ApiKey(ApiKeyAuth),
    /// OAuth 2.0 client credentials flow (HMRC application-restricted).
    ClientCredentials(ClientCredentialsAuth),
    /// OAuth 2.0 authorization code flow (HMRC user-restricted).
    AuthorizationCode(AuthorizationCodeAuth),
    // Future: GovernmentGateway variant for XML Gateway APIs (Phase 4+)
}

pub struct ApiKeyAuth {
    pub api_key: String,
}

pub struct ClientCredentialsAuth {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

pub struct AuthorizationCodeAuth {
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
    /// Port for the local callback server. `None` means bind to any available port.
    pub redirect_port: Option<u16>,
    pub scopes: Vec<String>,
}

const TOKEN_KEY_CLIENT_CREDS: &str = "oauth.client_credentials";
const TOKEN_KEY_AUTH_CODE: &str = "oauth.authorization_code";
const CALLBACK_TIMEOUT_SECS: u64 = 120;

/// Obtain an `Authorization` header for the given auth method.
///
/// For OAuth methods, tokens are cached in `token_store` and refreshed automatically.
/// Token exchange calls use the bare `reqwest::Client` (not `HttpClient`) to avoid
/// error-domain conflicts — see spec for rationale.
pub async fn authenticate(
    method: &AuthMethod,
    token_store: &TokenStore,
    http_client: &reqwest::Client,
    config: &mut AppConfig,
) -> crate::Result<HeaderMap> {
    match method {
        AuthMethod::ApiKey(auth) => Ok(basic_auth_header(&auth.api_key)),
        AuthMethod::ClientCredentials(auth) => {
            authenticate_client_credentials(auth, token_store, http_client).await
        }
        AuthMethod::AuthorizationCode(auth) => {
            authenticate_authorization_code(auth, token_store, http_client, config).await
        }
    }
}

async fn authenticate_client_credentials(
    auth: &ClientCredentialsAuth,
    token_store: &TokenStore,
    http_client: &reqwest::Client,
) -> crate::Result<HeaderMap> {
    if let Some(token) = token_store.get_valid_token(TOKEN_KEY_CLIENT_CREDS) {
        return Ok(bearer_header(&token));
    }

    let resp = oauth::exchange_client_credentials(
        http_client,
        &auth.token_url,
        &auth.client_id,
        &auth.client_secret,
        &auth.scopes,
    )
    .await?;

    token_store.store_token(
        TOKEN_KEY_CLIENT_CREDS,
        resp.access_token.clone(),
        resp.expires_in,
        None,
    );
    Ok(bearer_header(&resp.access_token))
}

async fn authenticate_authorization_code(
    auth: &AuthorizationCodeAuth,
    token_store: &TokenStore,
    http_client: &reqwest::Client,
    config: &mut AppConfig,
) -> crate::Result<HeaderMap> {
    // 1. Check for valid cached token
    if let Some(token) = token_store.get_valid_token(TOKEN_KEY_AUTH_CODE) {
        return Ok(bearer_header(&token));
    }

    // 2. Try refresh — from token store first, then credential store
    let refresh_token = token_store
        .get_refresh_token(TOKEN_KEY_AUTH_CODE)
        .or_else(|| get_credential(config, "hmrc.refresh_token").ok().flatten());

    if let Some(rt) = refresh_token {
        match oauth::refresh_access_token(
            http_client,
            &auth.token_url,
            &auth.client_id,
            &auth.client_secret,
            &rt,
        )
        .await
        {
            Ok(resp) => {
                let new_refresh = resp.refresh_token.clone();
                token_store.store_token(
                    TOKEN_KEY_AUTH_CODE,
                    resp.access_token.clone(),
                    resp.expires_in,
                    new_refresh.clone(),
                );
                if let Some(new_rt) = &new_refresh {
                    let _ = set_credential(config, "hmrc.refresh_token", new_rt, false);
                }
                return Ok(bearer_header(&resp.access_token));
            }
            Err(_) => {
                // Refresh failed — clear and fall through to full flow
                token_store.clear(TOKEN_KEY_AUTH_CODE);
                let _ = crate::config::delete_credential(config, "hmrc.refresh_token");
            }
        }
    }

    // 3. Full authorization code flow
    let state = generate_state();
    let port = auth.redirect_port.unwrap_or(0);
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .map_err(|e| AuthError::CallbackServerFailed {
            detail: e.to_string(),
        })?;
    let actual_port = listener
        .local_addr()
        .map_err(|e| AuthError::CallbackServerFailed {
            detail: e.to_string(),
        })?
        .port();
    let redirect_uri = format!("http://localhost:{actual_port}/callback");

    let scope_str = auth.scopes.join(" ");
    let mut authorize_url = reqwest::Url::parse(&auth.authorize_url).map_err(|e| {
        AuthError::CallbackServerFailed {
            detail: format!("invalid authorize URL: {e}"),
        }
    })?;
    authorize_url
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", &auth.client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("state", &state)
        .append_pair("scope", &scope_str);
    let authorize_url = authorize_url.to_string();

    eprintln!("Open this URL to authorize:\n  {authorize_url}");
    match open::that(&authorize_url) {
        Ok(()) => {}
        Err(e) => {
            warn!("Could not open browser: {e}");
            eprintln!("Please open the URL above in your browser manually.");
        }
    }

    let code =
        callback::wait_for_callback(listener, &state, Duration::from_secs(CALLBACK_TIMEOUT_SECS))
            .await?;

    let resp = oauth::exchange_authorization_code(
        http_client,
        &auth.token_url,
        &auth.client_id,
        &auth.client_secret,
        &code,
        &redirect_uri,
    )
    .await?;

    let new_refresh = resp.refresh_token.clone();
    token_store.store_token(
        TOKEN_KEY_AUTH_CODE,
        resp.access_token.clone(),
        resp.expires_in,
        new_refresh.clone(),
    );
    if let Some(rt) = &new_refresh {
        let _ = set_credential(config, "hmrc.refresh_token", rt, false);
    }

    Ok(bearer_header(&resp.access_token))
}

fn bearer_header(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).expect("valid header value"),
    );
    headers
}

fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
```

- [ ] **Step 4: Update lib.rs re-exports**

In `crates/alpha-gov-api-core/src/lib.rs`, update to:
```rust
pub mod auth;
pub mod config;
pub mod error;
pub mod http;
pub mod output;

pub use auth::{
    AuthMethod, ApiKeyAuth, AuthorizationCodeAuth, ClientCredentialsAuth, TokenStore,
    authenticate,
};
pub use config::{AppConfig, ConfigDisplay, ConfigFile, CredentialSource, Settings};
pub use error::{AppError, AuthError, ConfigError, HttpError, Result};
pub use http::{HttpClient, HttpClientConfig, HttpResponse};
pub use output::{ApiError, ApiErrorResponse, ApiMeta, ApiResponse, OutputFormat, print_json};
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `rustup run stable cargo test -p alpha-gov-api-core auth::tests`
Expected: all 4 tests PASS

Then run the full test suite:
Run: `rustup run stable cargo test -p alpha-gov-api-core`
Expected: all tests PASS (token_store, api_key, oauth, callback, auth::tests, config, http, output)

- [ ] **Step 6: Commit**

```bash
git add crates/alpha-gov-api-core/src/auth/mod.rs crates/alpha-gov-api-core/src/lib.rs
git commit -m "Add authenticate() entry point with client credentials and auth code flows (issue #4)"
```

---

### Task 7: Final verification and CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Run full test suite**

Run: `rustup run stable cargo test`
Expected: all tests PASS across both crates

- [ ] **Step 2: Run clippy**

Run: `rustup run stable cargo clippy`
Expected: no errors (warnings about unused items in binary crate are acceptable)

- [ ] **Step 3: Run format check**

Run: `rustup run stable cargo fmt --check`
Expected: no formatting issues

- [ ] **Step 4: Update CLAUDE.md**

Add to the Architecture section of `CLAUDE.md`, after the HTTP client paragraph:

```markdown
**Auth module** (`auth` submodule in core crate) — `authenticate()` dispatches to the correct flow based on `AuthMethod` enum: `ApiKey` (CH HTTP Basic), `ClientCredentials` (HMRC application-restricted OAuth), `AuthorizationCode` (HMRC user-restricted OAuth with local callback server). Token exchange calls use bare `reqwest::Client` (not `HttpClient`) to avoid error-domain conflicts. `TokenStore` caches tokens in-memory with 30s expiry buffer. Refresh tokens persisted to credential store.
```

Add to the Authentication patterns section:

```markdown
The `auth::authenticate()` function is the single entry point. It returns a `HeaderMap` with the `Authorization` header. Callers merge this into their request before sending via `HttpClient`.
```

- [ ] **Step 5: Commit**

```bash
git add CLAUDE.md
git commit -m "Document auth module in CLAUDE.md (issue #4)"
```
