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
        let params: HashMap<&str, &str> =
            query.split('&').filter_map(|p| p.split_once('=')).collect();

        // Check for OAuth error response
        if let Some(error) = params.get("error") {
            let desc = params.get("error_description").unwrap_or(&"unknown error");
            let html =
                format!("<html><body><h1>Authorization Failed</h1><p>{desc}</p></body></html>");
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
