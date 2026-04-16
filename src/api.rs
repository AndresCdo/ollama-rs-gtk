use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

use crate::config::Config;

pub async fn send_prompt(prompt: &str) -> Result<String> {
    let config = Config::load().unwrap_or_default();

    let client = Client::builder()
        .timeout(Duration::from_secs(config.request_timeout))
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .post(&config.api_url)
        .json(&json!({
            "model": config.model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .context("Failed to send request")?;

    let json: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;
    Ok(json["response"]
        .as_str()
        .unwrap_or("Error: No response")
        .to_string())
}
