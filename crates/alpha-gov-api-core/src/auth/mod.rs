pub mod api_key;
pub mod callback;
pub mod oauth;
pub mod token_store;

pub use api_key::basic_auth_header;
pub use oauth::TokenResponse;
pub use token_store::TokenStore;

use std::time::Duration;

use rand::RngCore;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use tracing::warn;

use crate::AppConfig;
use crate::config::{get_credential, set_credential};
use crate::error::AuthError;

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
    let mut authorize_url =
        reqwest::Url::parse(&auth.authorize_url).map_err(|e| AuthError::CallbackServerFailed {
            detail: format!("invalid authorize URL: {e}"),
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

        let auth_method = AuthMethod::ClientCredentials(ClientCredentialsAuth {
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
        let headers = authenticate(&auth_method, &store, &client, &mut config)
            .await
            .unwrap();
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert_eq!(auth, "Bearer cached-tok");

        // Second call — cache hit, no second request
        let headers2 = authenticate(&auth_method, &store, &client, &mut config)
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
