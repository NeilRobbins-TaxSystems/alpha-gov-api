use std::path::PathBuf;

/// Alias for results using [`AppError`].
pub type Result<T> = std::result::Result<T, AppError>;

/// Top-level error type for the application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Config(Box<ConfigError>),

    #[error(transparent)]
    Http(Box<HttpError>),

    #[error(transparent)]
    Auth(Box<AuthError>),
}

/// Configuration and credential errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("cannot determine config directory for this platform")]
    NoConfigDir,

    #[error("failed to read config file {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to write config file {path}: {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("invalid TOML in {path}: {source}")]
    ParseToml {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to serialize config: {source}")]
    SerializeToml { source: toml::ser::Error },

    #[error("keychain error: {detail}")]
    Keychain { detail: String },

    #[error("output error: {source}")]
    Output { source: serde_json::Error },
}

/// HTTP client and API errors.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("failed to build HTTP client: {source}")]
    BuildClient { source: reqwest::Error },

    #[error("network error for {url}: {source}")]
    Network { url: String, source: reqwest::Error },

    #[error("request timed out: {url}")]
    Timeout { url: String },

    #[error("authentication error {status} for {url}")]
    Auth { status: u16, url: String },

    #[error("API error {status} for {url}: {body}")]
    Api {
        status: u16,
        url: String,
        body: String,
    },

    #[error("max retries ({attempts}) exceeded for {url}")]
    MaxRetriesExceeded { url: String, attempts: u32 },
}

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

impl AppError {
    /// Downcast to `HttpError` if this is the `Http` variant.
    pub fn downcast_http(self) -> Option<Box<HttpError>> {
        match self {
            AppError::Http(e) => Some(e),
            _ => None,
        }
    }

    /// Downcast to `AuthError` if this is the `Auth` variant.
    pub fn downcast_auth(self) -> Option<Box<AuthError>> {
        match self {
            AppError::Auth(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(Box::new(e))
    }
}

impl From<HttpError> for AppError {
    fn from(e: HttpError) -> Self {
        AppError::Http(Box::new(e))
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        AppError::Auth(Box::new(e))
    }
}
