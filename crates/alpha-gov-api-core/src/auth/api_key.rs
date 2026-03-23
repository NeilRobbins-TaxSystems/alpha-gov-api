use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

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
