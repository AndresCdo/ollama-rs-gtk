use anyhow::{bail, Context, Result};
use reqwest::Url;
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;
use std::time::Duration;

const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointKind {
    Chat,
    Generate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    request_timeout: Option<u64>,
    #[serde(default)]
    request_timeout_ms: Option<u64>,
    #[serde(default)]
    request_timeout_secs: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: default_api_url(),
            model: default_model(),
            request_timeout: None,
            request_timeout_ms: None,
            request_timeout_secs: Some(DEFAULT_REQUEST_TIMEOUT_SECS),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let contents = match fs::read_to_string("config.toml") {
            Ok(contents) => contents,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => return Err(error).context("Failed to read config.toml"),
        };

        let config: Self = toml::from_str(&contents).context("Failed to parse config.toml")?;
        Ok(config)
    }

    pub fn request_timeout(&self) -> Duration {
        if let Some(seconds) = self.request_timeout_secs {
            Duration::from_secs(seconds)
        } else if let Some(milliseconds) = self.request_timeout_ms.or(self.request_timeout) {
            Duration::from_millis(milliseconds)
        } else {
            Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_SECS)
        }
    }

    pub fn endpoint_kind(&self) -> Result<EndpointKind> {
        let url = Url::parse(&self.api_url).context("Invalid api_url in config.toml")?;
        let path = url.path().trim_end_matches('/');

        match path {
            "" | "/api" | "/api/chat" => Ok(EndpointKind::Chat),
            "/api/generate" => Ok(EndpointKind::Generate),
            _ => bail!(
                "Unsupported api_url path '{}'. Use /api, /api/chat, or /api/generate",
                url.path()
            ),
        }
    }

    pub fn resolved_api_url(&self) -> Result<String> {
        let mut url = Url::parse(&self.api_url).context("Invalid api_url in config.toml")?;

        match self.endpoint_kind()? {
            EndpointKind::Chat => url.set_path("/api/chat"),
            EndpointKind::Generate => url.set_path("/api/generate"),
        }

        Ok(url.to_string())
    }
}

fn default_api_url() -> String {
    "http://localhost:11434/api".to_string()
}

fn default_model() -> String {
    "llama3.1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_current_behavior() {
        let config = Config::default();
        assert_eq!(config.api_url, "http://localhost:11434/api");
        assert_eq!(config.model, "llama3.1");
        assert_eq!(config.request_timeout(), Duration::from_secs(60));
        assert_eq!(config.endpoint_kind().unwrap(), EndpointKind::Chat);
        assert_eq!(
            config.resolved_api_url().unwrap(),
            "http://localhost:11434/api/chat"
        );
    }

    #[test]
    fn legacy_generate_endpoint_is_still_supported() {
        let config = Config {
            api_url: "http://localhost:11434/api/generate".to_string(),
            model: "llama3.1".to_string(),
            request_timeout: Some(10_000),
            request_timeout_ms: None,
            request_timeout_secs: None,
        };

        assert_eq!(config.endpoint_kind().unwrap(), EndpointKind::Generate);
        assert_eq!(
            config.resolved_api_url().unwrap(),
            "http://localhost:11434/api/generate"
        );
        assert_eq!(config.request_timeout(), Duration::from_millis(10_000));
    }

    #[test]
    fn base_api_url_resolves_to_chat_endpoint() {
        let config = Config {
            api_url: "http://localhost:11434".to_string(),
            ..Config::default()
        };

        assert_eq!(config.endpoint_kind().unwrap(), EndpointKind::Chat);
        assert_eq!(
            config.resolved_api_url().unwrap(),
            "http://localhost:11434/api/chat"
        );
    }
}
