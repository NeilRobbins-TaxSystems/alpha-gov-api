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
        if let Some(expires_at) = entry.expires_at
            && Instant::now() + Duration::from_secs(EXPIRY_BUFFER_SECS) >= expires_at
        {
            return None;
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
        store.store_token(
            "hmrc",
            "access".into(),
            Some(3600),
            Some("refresh-tok".into()),
        );
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
