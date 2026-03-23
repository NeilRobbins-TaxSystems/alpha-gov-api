use chrono::{DateTime, Utc};
use serde::Serialize;

/// Successful API response envelope.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    ok: bool,
    pub data: T,
    pub meta: ApiMeta,
}

/// Metadata included with every successful response.
#[derive(Debug, Serialize)]
pub struct ApiMeta {
    pub api: String,
    pub endpoint: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_remaining: Option<u32>,
}

/// Error response envelope.
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    ok: bool,
    pub error: ApiError,
}

/// Structured error details.
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T, api: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            ok: true,
            data,
            meta: ApiMeta {
                api: api.into(),
                endpoint: endpoint.into(),
                timestamp: Utc::now(),
                rate_limit_remaining: None,
            },
        }
    }
}

impl ApiErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            ok: false,
            error: ApiError {
                code: code.into(),
                message: message.into(),
                api_status: None,
                api: None,
            },
        }
    }

    pub fn with_api(mut self, api: impl Into<String>, status: u16) -> Self {
        self.error.api = Some(api.into());
        self.error.api_status = Some(status);
        self
    }
}

/// Output format for CLI rendering.
#[derive(Debug, Clone, Copy, Default)]
pub enum OutputFormat {
    /// Compact JSON (single line, no extra whitespace). This is the default.
    #[default]
    Json,
    /// Pretty-printed JSON (indented).
    Pretty,
}

/// Serialise a value to stdout in the given format.
///
/// Returns an error if serialisation fails.
pub fn print_json<T: Serialize>(value: &T, format: OutputFormat) -> serde_json::Result<()> {
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    match format {
        OutputFormat::Json => serde_json::to_writer(&mut handle, value)?,
        OutputFormat::Pretty => serde_json::to_writer_pretty(&mut handle, value)?,
    }
    handle.write_all(b"\n").map_err(serde_json::Error::io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_envelope_shape() {
        let resp = ApiResponse::new("hello", "test-api", "/test");
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["ok"], true);
        assert_eq!(json["data"], "hello");
        assert!(json["meta"]["timestamp"].is_string());
        assert!(json["meta"].get("rate_limit_remaining").is_none());
    }

    #[test]
    fn error_envelope_shape() {
        let resp = ApiErrorResponse::new("NOT_FOUND", "Company not found")
            .with_api("companies-house", 404);
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["ok"], false);
        assert_eq!(json["error"]["code"], "NOT_FOUND");
        assert_eq!(json["error"]["api_status"], 404);
        assert_eq!(json["error"]["api"], "companies-house");
    }

    #[test]
    fn error_envelope_without_api() {
        let resp = ApiErrorResponse::new("INVALID_INPUT", "Missing required field");
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["ok"], false);
        assert!(json["error"].get("api_status").is_none());
        assert!(json["error"].get("api").is_none());
    }
}
