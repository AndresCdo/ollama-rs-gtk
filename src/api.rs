use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const REQUEST_TIMEOUT: u64 = 10000;

pub async fn send_prompt(prompt: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .post(OLLAMA_API_URL)
        .json(&json!({
            "model": "llama3.1",
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
