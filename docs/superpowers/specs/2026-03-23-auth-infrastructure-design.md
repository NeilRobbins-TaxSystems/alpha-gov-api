# Auth Infrastructure Design (Issue #4)

## Overview

Authentication flows for all API providers in alpha-gov-api. Covers API key auth (Companies House), OAuth 2.0 client credentials (HMRC application-restricted), OAuth 2.0 authorization code with local callback server (HMRC user-restricted), and in-memory token caching with refresh. Government Gateway handling is deferred to later phases.

## Module Structure

```
crates/alpha-gov-api-core/src/auth/
  mod.rs          - AuthMethod enum, authenticate() entry point, re-exports
  api_key.rs      - CH API key -> HTTP Basic header
  oauth.rs        - Token exchange (client credentials + auth code), refresh
  token_store.rs  - Thread-safe in-memory token cache with TTL
  callback.rs     - Local HTTP server for OAuth redirect
```

New module `auth` added to `lib.rs` alongside existing `config`, `error`, `http`, `output`.

## Core Types

### AuthMethod enum

```rust
pub enum AuthMethod {
    /// API key sent as HTTP Basic username with empty password (Companies House).
    ApiKey(ApiKeyAuth),
    /// OAuth 2.0 client credentials flow (HMRC application-restricted).
    ClientCredentials(ClientCredentialsAuth),
    /// OAuth 2.0 authorization code flow (HMRC user-restricted).
    AuthorizationCode(AuthorizationCodeAuth),
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
    /// Note: HMRC requires redirect URIs to be pre-registered, so a fixed port
    /// (e.g. 9004) is typical for production use. Ephemeral (None/0) is for tests.
    pub redirect_port: Option<u16>,
    pub scopes: Vec<String>,
}
```

### TokenStore

Thread-safe in-memory cache keyed by provider identifier (e.g. `"hmrc"`):

```rust
pub struct TokenStore {
    tokens: Mutex<HashMap<String, TokenEntry>>,
}

struct TokenEntry {
    access_token: String,
    expires_at: Option<Instant>,
    refresh_token: Option<String>,
}
```

- `get_valid_token(key)` returns `Some(access_token)` only if the token exists and is not expired (30-second buffer before actual expiry to avoid edge-case failures).
- `store_token(key, access_token, expires_in_secs, refresh_token)` caches the token.
- Refresh tokens are also persisted to the credential store (via existing `set_credential`) so they survive process restarts.
- Access tokens are in-memory only (short-lived, typically 4 hours for HMRC).

## Public API

### authenticate()

```rust
pub async fn authenticate(
    method: &AuthMethod,
    token_store: &TokenStore,
    http_client: &reqwest::Client,
    config: &AppConfig,
) -> Result<HeaderMap>
```

**Why `&reqwest::Client` and not `&HttpClient`:** Token exchange and refresh calls are one-shot operations that must not use `HttpClient::send()`. The `HttpClient` wrapper treats 401 as `HttpError::Auth` (a non-retryable HTTP-level error), but a 401 from a token endpoint means "bad credentials" and must map to `AuthError::TokenExchangeFailed` instead. Using the bare `reqwest::Client` lets the auth module inspect responses directly and produce the correct `AuthError` variants. Callers pass `client.inner()`.

Returns an `Authorization` header ready to merge into a request:
- `ApiKey` -> `Authorization: Basic base64(api_key + ":")`
- `ClientCredentials` -> `Authorization: Bearer <access_token>` (obtained/cached via token endpoint)
- `AuthorizationCode` -> `Authorization: Bearer <access_token>` (obtained via auth code flow or refreshed)

Caller usage:
```rust
let auth_headers = auth::authenticate(&method, &store, client.inner(), &config).await?;
let req = client.inner().get(url).headers(auth_headers).build()?;
let resp = client.send(req).await?;
```

## Auth Flows

### API Key (Companies House)

1. Take `api_key` from `ApiKeyAuth`.
2. Base64-encode `"{api_key}:"` (key as username, empty password).
3. Return `Authorization: Basic {encoded}` header.

No token caching needed — the key is static.

### OAuth 2.0 Client Credentials (HMRC Application-Restricted)

1. Check `TokenStore` for a valid cached token for this provider.
2. If valid token exists, return `Authorization: Bearer {token}`.
3. Otherwise, POST to `token_url` with:
   - `grant_type=client_credentials`
   - `client_id={client_id}`
   - `client_secret={client_secret}`
   - `scope={scopes joined by space}` (if any)
4. Parse JSON response: `{ "access_token", "expires_in", "token_type" }`.
5. Store token in `TokenStore` with expiry.
6. Return `Authorization: Bearer {token}`.

### OAuth 2.0 Authorization Code (HMRC User-Restricted)

1. Check `TokenStore` for a valid cached token.
2. If valid, return it.
3. If no cached token, attempt to load refresh token from credential store via `get_credential(config, "hmrc.refresh_token")` and attempt refresh (see below).
4. If expired but `TokenStore` has a refresh token, attempt refresh (see below).
5. If no token and no refresh token, run the full authorization code flow:

**Full flow:**
1. Generate random 32-byte hex `state` parameter (CSRF protection).
2. Bind `tokio::net::TcpListener` on `127.0.0.1:{redirect_port}` (or port 0 if `None`). Record the actual bound port for the redirect URI.
3. Build authorize URL: `{authorize_url}?response_type=code&client_id={id}&redirect_uri=http://localhost:{port}/callback&state={state}&scope={scopes}`.
4. Print the URL to stderr (for headless environments).
5. Open browser via `open::that()`.
6. Loop: accept TCP connections. For each connection, read the HTTP request line. If the request path does not start with `/callback` or does not contain a `code` query parameter (e.g. browser favicon request), respond with 404 and continue listening. This handles browsers that send ancillary requests before/after the callback.
7. When a request to `/callback` with a `code` parameter arrives, parse the full query string.
8. Verify `state` matches the generated value; extract `code`. If `error` parameter present, return `AuthError`.
9. Write HTTP 200 response with HTML success page ("You can close this tab"), close connection, drop listener.
10. Exchange code for tokens: POST to `token_url` with `grant_type=authorization_code`, `code`, `redirect_uri`, `client_id`, `client_secret`.
11. Parse JSON response: `{ "access_token", "expires_in", "refresh_token", "token_type" }`.
12. Cache access token in `TokenStore`.
13. Persist refresh token to credential store via `set_credential(config, "hmrc.refresh_token", token, false)` (keychain preferred).
14. Return `Authorization: Bearer {token}`.

**Refresh flow:**
1. POST to `token_url` with `grant_type=refresh_token`, `refresh_token`, `client_id`, `client_secret`.
2. Parse response; update cached access token and (if returned) refresh token.
3. If refresh fails with 4xx, clear stored refresh token and fall through to full flow.

### Timeout

The callback server waits up to 120 seconds for the browser redirect. If no connection is received, return `AuthError::CallbackServerFailed` with a timeout message.

## Error Handling

New `AuthError` enum following the project's boxed sub-enum pattern:

```rust
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

Added to `AppError`:
```rust
#[error(transparent)]
Auth(Box<AuthError>),
```

With `From<AuthError> for AppError` producing `AppError::Auth(Box::new(e))`.

Add `downcast_auth()` helper on `AppError` (analogous to existing `downcast_http()`) for use in tests:
```rust
pub fn downcast_auth(self) -> Option<Box<AuthError>> {
    match self {
        AppError::Auth(e) => Some(e),
        _ => None,
    }
}
```

**Note on `AppError::Auth` vs `HttpError::Auth`:** These are distinct error domains. `HttpError::Auth` means "the API returned 401/403 for a data request." `AppError::Auth(AuthError)` means "the authentication/token infrastructure itself failed." They do not conflict because token exchange calls use `reqwest::Client` directly (not `HttpClient`), so token-endpoint 401s never become `HttpError::Auth`.

## Config Changes

Add `hmrc.refresh_token` to `CREDENTIAL_KEYS` in `config.rs`:
```rust
("hmrc.refresh_token", "ALPHA_GOV_API_HMRC_REFRESH_TOKEN"),
```
This ensures `config show` displays its status and allows env-var override.

## TokenStore Ownership

The `TokenStore` is constructed in the CLI binary's `run()` function and passed by reference to `authenticate()`. It lives for the duration of a single CLI invocation. Since CLI commands are short-lived processes, the in-memory cache primarily helps commands that make multiple authenticated requests in one run (e.g. paginated API calls). No `Arc` or cross-process sharing is needed.

## New Dependencies

Added to `[workspace.dependencies]` in root `Cargo.toml`:

- `base64 = "0.22"` — HTTP Basic auth encoding
- `rand = "0.9"` — CSRF state parameter generation
- `open = "5"` — cross-platform browser launch

All referenced via `{ workspace = true }` in the core crate's `Cargo.toml`.

## Testing Strategy

### Unit tests (no network)

- **API key**: `api_key_produces_correct_basic_header` — verify Base64 encoding matches `base64("{key}:")`.
- **Token store**: `token_store_returns_none_when_expired`, `token_store_returns_valid_token`, `token_store_overwrites_on_store` — verify TTL logic and the 30s expiry buffer.
- **State generation**: verify random state is 64 hex characters.

### Integration tests (wiremock)

- **Client credentials exchange**: mock token endpoint returns `{ "access_token": "tok", "expires_in": 3600 }`, verify `authenticate()` returns correct Bearer header and caches the token. Second call hits cache, no second request.
- **Client credentials failure**: mock returns 401, verify `AuthError::TokenExchangeFailed`.
- **Token refresh**: pre-populate token store with expired token + refresh token, mock refresh endpoint, verify new token cached.
- **Token refresh failure**: mock returns 400 on refresh, verify fallback behavior (clears refresh token).
- **Auth code token exchange**: mock token endpoint, call the token exchange function directly with a known code, verify correct Bearer header.

### Callback server test

- Start callback server on an ephemeral port (`port 0`).
- Spawn a task that sends a mock HTTP GET with `?code=test123&state={expected}` to the listener.
- Verify the server returns the code and shuts down.
- Test state mismatch: send wrong state, verify `AuthError::InvalidState`.
- Test timeout: start server with short timeout, verify `AuthError::CallbackServerFailed`.

### What is NOT tested

- Actual browser launch (`open::that`) — side-effect, mocked at integration boundary.
- Actual keychain writes — already tested in config module.
- Real HMRC/CH endpoints — requires live credentials, covered by manual testing with `--sandbox`.

## Government Gateway

Deferred to later phases per issue notes. A comment in `AuthMethod` marks the future variant:
```rust
// Future: GovernmentGateway variant for XML Gateway APIs (Phase 4+)
```

## Sandbox Support

Auth types take full URLs (token endpoint, authorize endpoint), so callers pass sandbox or production URLs based on `config.sandbox`. URL construction is the caller's responsibility — the auth module is URL-agnostic.

HMRC endpoints:
- Production: `https://api.service.hmrc.gov.uk/oauth/token`, `/oauth/authorize`
- Sandbox: `https://test-api.service.hmrc.gov.uk/oauth/token`, `/oauth/authorize`
