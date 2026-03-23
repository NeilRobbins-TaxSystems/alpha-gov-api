use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::error::{ConfigError, Result};

const APP_NAME: &str = "alpha-gov-api";
const ENV_PREFIX: &str = "ALPHA_GOV_API_";

/// Known credential keys and their environment variable names.
const CREDENTIAL_KEYS: &[(&str, &str)] = &[
    ("ch.api_key", "ALPHA_GOV_API_CH_KEY"),
    ("hmrc.client_id", "ALPHA_GOV_API_HMRC_CLIENT_ID"),
    ("hmrc.client_secret", "ALPHA_GOV_API_HMRC_CLIENT_SECRET"),
    ("hmrc.server_token", "ALPHA_GOV_API_HMRC_SERVER_TOKEN"),
    ("hmrc.refresh_token", "ALPHA_GOV_API_HMRC_REFRESH_TOKEN"),
    ("govgateway.user_id", "ALPHA_GOV_API_GOVGATEWAY_USER_ID"),
    ("govgateway.password", "ALPHA_GOV_API_GOVGATEWAY_PASSWORD"),
];

// ---------------------------------------------------------------------------
// Config file types
// ---------------------------------------------------------------------------

/// Top-level configuration file structure.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    /// Default profile settings.
    #[serde(default)]
    pub defaults: Settings,

    /// Named profiles override defaults.
    #[serde(default)]
    pub profile: BTreeMap<String, Settings>,

    /// Plaintext credential fallback (used when keychain is unavailable).
    #[serde(default)]
    pub credentials: BTreeMap<String, String>,
}

/// Settings that can appear at the top level or within a profile.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub sandbox: Option<bool>,
    pub output_pretty: Option<bool>,
    pub quiet: Option<bool>,

    /// Arbitrary provider-specific settings.
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

// ---------------------------------------------------------------------------
// Config resolution
// ---------------------------------------------------------------------------

/// Resolved configuration after merging file + profile + env.
#[derive(Debug)]
pub struct AppConfig {
    pub config_path: PathBuf,
    pub profile_name: Option<String>,
    pub sandbox: bool,
    pub file: ConfigFile,
}

impl AppConfig {
    /// Load configuration from disk, applying the given profile.
    ///
    /// Resolution order (highest priority wins):
    /// 1. Environment variables (`ALPHA_GOV_API_*`)
    /// 2. Profile section in config file
    /// 3. Defaults section in config file
    pub fn load(config_path: Option<&Path>, profile: Option<&str>) -> Result<Self> {
        let path = match config_path {
            Some(p) => p.to_path_buf(),
            None => default_config_path()?,
        };

        let file = if path.exists() {
            let contents = fs::read_to_string(&path).map_err(|e| ConfigError::ReadFile {
                path: path.clone(),
                source: e,
            })?;
            toml::from_str(&contents).map_err(|e| ConfigError::ParseToml {
                path: path.clone(),
                source: e,
            })?
        } else {
            ConfigFile::default()
        };

        let sandbox = resolve_bool(&file, profile, "sandbox", "SANDBOX");

        Ok(Self {
            config_path: path,
            profile_name: profile.map(String::from),
            sandbox,
            file,
        })
    }

    /// Persist the current config file to disk.
    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| ConfigError::WriteFile {
                path: self.config_path.clone(),
                source: e,
            })?;
        }
        let contents = toml::to_string_pretty(&self.file)
            .map_err(|e| ConfigError::SerializeToml { source: e })?;
        fs::write(&self.config_path, contents).map_err(|e| ConfigError::WriteFile {
            path: self.config_path.clone(),
            source: e,
        })?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Credential resolution
// ---------------------------------------------------------------------------

/// Retrieve a credential value. Resolution order:
/// 1. Environment variable
/// 2. OS keychain (via `keyring`)
/// 3. Plaintext in config file
pub fn get_credential(config: &AppConfig, key: &str) -> Result<Option<String>> {
    // 1. Environment variable
    if let Some(env_var) = env_var_for_key(key)
        && let Ok(val) = env::var(env_var)
        && !val.is_empty()
    {
        return Ok(Some(val));
    }

    // 2. OS keychain
    match keyring_get(key) {
        Ok(Some(val)) => return Ok(Some(val)),
        Ok(None) => {}
        Err(_) => {
            // Keychain unavailable — fall through to plaintext
        }
    }

    // 3. Plaintext fallback
    Ok(config.file.credentials.get(key).cloned())
}

/// Store a credential. By default uses OS keychain; with `plaintext=true` stores in config TOML.
pub fn set_credential(
    config: &mut AppConfig,
    key: &str,
    value: &str,
    plaintext: bool,
) -> Result<()> {
    if plaintext {
        config
            .file
            .credentials
            .insert(key.to_string(), value.to_string());
        config.save()?;
    } else {
        keyring_set(key, value)?;
    }
    Ok(())
}

/// Delete a credential from all stores.
pub fn delete_credential(config: &mut AppConfig, key: &str) -> Result<()> {
    config.file.credentials.remove(key);
    config.save()?;
    let _ = keyring_delete(key);
    Ok(())
}

// ---------------------------------------------------------------------------
// Config display (for `config show`)
// ---------------------------------------------------------------------------

/// Produce a redacted view of the configuration for display.
pub fn config_display(config: &AppConfig) -> ConfigDisplay {
    let mut credentials = BTreeMap::new();
    for (key, env_var) in CREDENTIAL_KEYS {
        let source = if env::var(env_var).is_ok_and(|v| !v.is_empty()) {
            CredentialSource::EnvVar
        } else if keyring_get(key).ok().flatten().is_some() {
            CredentialSource::Keychain
        } else if config.file.credentials.contains_key(*key) {
            CredentialSource::ConfigFile
        } else {
            CredentialSource::NotSet
        };
        credentials.insert((*key).to_string(), source);
    }

    ConfigDisplay {
        config_path: config.config_path.display().to_string(),
        active_profile: config.profile_name.clone(),
        sandbox: config.sandbox,
        credentials,
        profiles: config.file.profile.keys().cloned().collect(),
    }
}

#[derive(Debug, Serialize)]
pub struct ConfigDisplay {
    pub config_path: String,
    pub active_profile: Option<String>,
    pub sandbox: bool,
    pub credentials: BTreeMap<String, CredentialSource>,
    pub profiles: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialSource {
    EnvVar,
    Keychain,
    ConfigFile,
    NotSet,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Default config file path using platform conventions.
pub fn default_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
    Ok(config_dir.join(APP_NAME).join("config.toml"))
}

/// Resolve a boolean setting: env > profile > defaults.
fn resolve_bool(file: &ConfigFile, profile: Option<&str>, field: &str, env_suffix: &str) -> bool {
    // Env var override
    let env_key = format!("{ENV_PREFIX}{env_suffix}");
    if let Ok(val) = env::var(&env_key) {
        match val.to_lowercase().as_str() {
            "1" | "true" | "yes" => return true,
            "0" | "false" | "no" => return false,
            _ => {}
        }
    }

    // Profile override
    if let Some(name) = profile
        && let Some(prof) = file.profile.get(name)
        && let Some(val) = get_bool_field(prof, field)
    {
        return val;
    }

    // Defaults
    get_bool_field(&file.defaults, field).unwrap_or(false)
}

fn get_bool_field(settings: &Settings, field: &str) -> Option<bool> {
    match field {
        "sandbox" => settings.sandbox,
        "output_pretty" => settings.output_pretty,
        "quiet" => settings.quiet,
        _ => None,
    }
}

fn env_var_for_key(key: &str) -> Option<&'static str> {
    CREDENTIAL_KEYS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

const KEYRING_SERVICE: &str = "alpha-gov-api";

fn keyring_get(key: &str) -> Result<Option<String>> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, key).map_err(|e| ConfigError::Keychain {
        detail: e.to_string(),
    })?;
    match entry.get_password() {
        Ok(val) => Ok(Some(val)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(ConfigError::Keychain {
            detail: e.to_string(),
        }
        .into()),
    }
}

fn keyring_set(key: &str, value: &str) -> Result<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, key).map_err(|e| ConfigError::Keychain {
        detail: e.to_string(),
    })?;
    entry
        .set_password(value)
        .map_err(|e| ConfigError::Keychain {
            detail: e.to_string(),
        })?;
    Ok(())
}

fn keyring_delete(key: &str) -> Result<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, key).map_err(|e| ConfigError::Keychain {
        detail: e.to_string(),
    })?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(ConfigError::Keychain {
            detail: e.to_string(),
        }
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn load_empty_config() {
        let config = AppConfig::load(Some(Path::new("/nonexistent/path.toml")), None).unwrap();
        assert!(!config.sandbox);
        assert!(config.file.profile.is_empty());
        assert!(config.file.credentials.is_empty());
    }

    #[test]
    fn load_config_with_profiles() {
        let toml_content = r#"
[defaults]
sandbox = false

[profile.test]
sandbox = true

[credentials]
"ch.api_key" = "test-key-123"
"#;
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{toml_content}").unwrap();

        let config = AppConfig::load(Some(tmp.path()), Some("test")).unwrap();
        assert!(config.sandbox);
        assert_eq!(
            config.file.credentials.get("ch.api_key").unwrap(),
            "test-key-123"
        );
    }

    #[test]
    fn load_config_default_profile() {
        let toml_content = r#"
[defaults]
sandbox = true
"#;
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{toml_content}").unwrap();

        let config = AppConfig::load(Some(tmp.path()), None).unwrap();
        assert!(config.sandbox);
    }

    #[test]
    fn env_var_overrides_file() {
        let toml_content = r#"
[defaults]
sandbox = false
"#;
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{toml_content}").unwrap();

        // SAFETY: This test is not run in parallel with other tests that read this env var.
        unsafe { env::set_var("ALPHA_GOV_API_SANDBOX", "true") };
        let config = AppConfig::load(Some(tmp.path()), None).unwrap();
        assert!(config.sandbox);
        unsafe { env::remove_var("ALPHA_GOV_API_SANDBOX") };
    }

    #[test]
    fn plaintext_credential_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let mut config = AppConfig::load(Some(&path), None).unwrap();
        set_credential(&mut config, "ch.api_key", "my-secret", true).unwrap();

        // Reload and verify
        let config2 = AppConfig::load(Some(&path), None).unwrap();
        let cred = get_credential(&config2, "ch.api_key").unwrap();
        assert_eq!(cred.unwrap(), "my-secret");
    }

    #[test]
    fn config_display_shows_not_set() {
        let config = AppConfig::load(Some(Path::new("/nonexistent/path.toml")), None).unwrap();
        let display = config_display(&config);
        for (_key, source) in &display.credentials {
            assert!(matches!(source, CredentialSource::NotSet));
        }
    }

    #[test]
    fn save_and_reload() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_path_buf();

        let mut config = AppConfig::load(Some(&path), None).unwrap();
        config.file.defaults.sandbox = Some(true);
        config
            .file
            .profile
            .insert("prod".to_string(), Settings::default());
        config.save().unwrap();

        let config2 = AppConfig::load(Some(&path), None).unwrap();
        assert_eq!(config2.file.defaults.sandbox, Some(true));
        assert!(config2.file.profile.contains_key("prod"));
    }
}
