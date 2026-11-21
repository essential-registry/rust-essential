use crate::{Error, Result};

/// Application configuration, read from environment variables.
///
/// Every variable is documented in `.env.example`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Name used in greetings and logs (`APP_NAME`).
    pub app_name: String,
    /// Log filter in `tracing` syntax (`LOG_LEVEL`).
    pub log_level: String,
}

impl Config {
    /// Read configuration from the process environment.
    pub fn from_env() -> Result<Self> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Build configuration from any key lookup. Tests use this to avoid touching the environment.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self> {
        let app_name = lookup("APP_NAME").unwrap_or_else(|| "Rust Essential".to_owned());
        if app_name.trim().is_empty() {
            return Err(Error::Config("APP_NAME must not be empty".into()));
        }
        Ok(Self {
            app_name,
            log_level: lookup("LOG_LEVEL").unwrap_or_else(|| "info".to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_apply_when_unset() {
        let config = Config::from_lookup(|_| None).unwrap();
        assert_eq!(config.app_name, "Rust Essential");
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn values_are_read() {
        let config =
            Config::from_lookup(|key| (key == "APP_NAME").then(|| "Demo".to_owned())).unwrap();
        assert_eq!(config.app_name, "Demo");
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = Config::from_lookup(|key| (key == "APP_NAME").then(|| "  ".to_owned()));
        assert!(matches!(result, Err(Error::Config(_))));
    }
}
