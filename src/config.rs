use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
            model: default_model(),
            request_timeout: default_request_timeout(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let contents = fs::read_to_string("config.toml").context("Failed to read config.toml")?;
        let config: Self = toml::from_str(&contents).context("Failed to parse config.toml")?;
        Ok(config)
    }
}

fn default_api_url() -> String {
    "http://localhost:11434/api/generate".to_string()
}

fn default_model() -> String {
    "llama3.1".to_string()
}

fn default_request_timeout() -> u64 {
    10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_current_behavior() {
        let config = Config::default();
        assert_eq!(config.api_url, "http://localhost:11434/api/generate");
        assert_eq!(config.model, "llama3.1");
        assert_eq!(config.request_timeout, 10_000);
    }
}
