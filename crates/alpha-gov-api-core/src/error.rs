use std::path::PathBuf;

/// Alias for results using [`AppError`].
pub type Result<T> = std::result::Result<T, AppError>;

/// Top-level error type for the application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Config(Box<ConfigError>),
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

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(Box::new(e))
    }
}
