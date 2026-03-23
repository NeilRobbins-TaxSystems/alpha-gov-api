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

    resp.json()
        .await
        .map_err(|e| AuthError::InvalidTokenResponse {
            detail: e.to_string(),
        })
}

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
            .respond_with(ResponseTemplate::new(401).set_body_string("invalid_client"))
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
            .and(body_string_contains(
                "redirect_uri=http%3A%2F%2Flocalhost%3A9004%2Fcallback",
            ))
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
            .respond_with(ResponseTemplate::new(400).set_body_string("invalid_grant"))
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
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
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
