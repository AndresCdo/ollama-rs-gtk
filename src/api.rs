use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::{Config, EndpointKind};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: Option<ChatResponseMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: String,
}

pub async fn send_prompt(prompt: &str, history: &[ChatMessage]) -> Result<String> {
    let config = Config::load()?;
    let request_url = config.resolved_api_url()?;
    let endpoint_kind = config.endpoint_kind()?;

    let client = Client::builder()
        .timeout(config.request_timeout())
        .build()
        .context("Failed to build HTTP client")?;

    let response = match endpoint_kind {
        EndpointKind::Chat => {
            let mut messages = history.to_vec();
            messages.push(ChatMessage::user(prompt));

            client
                .post(request_url)
                .json(&json!({
                    "model": config.model,
                    "messages": messages,
                    "stream": false
                }))
                .send()
                .await
        }
        EndpointKind::Generate => {
            client
                .post(request_url)
                .json(&json!({
                    "model": config.model,
                    "prompt": prompt,
                    "stream": false
                }))
                .send()
                .await
        }
    }
    .context("Failed to send request")?;

    extract_response_text(response, endpoint_kind).await
}

async fn extract_response_text(response: Response, endpoint_kind: EndpointKind) -> Result<String> {
    let status = response.status();
    let body = response
        .text()
        .await
        .context("Failed to read Ollama response body")?;

    if !status.is_success() {
        let error = extract_api_error(&body).unwrap_or_else(|| body.trim().to_string());
        return Err(anyhow!(
            "Ollama API request failed with {}: {}",
            status,
            error
        ));
    }

    match endpoint_kind {
        EndpointKind::Chat => {
            let response: ChatResponse =
                serde_json::from_str(&body).context("Failed to parse chat response JSON")?;

            response
                .message
                .map(|message| message.content)
                .filter(|content| !content.trim().is_empty())
                .ok_or_else(|| anyhow!("Ollama chat response did not contain message content"))
        }
        EndpointKind::Generate => {
            let response: GenerateResponse =
                serde_json::from_str(&body).context("Failed to parse generate response JSON")?;

            response
                .response
                .filter(|content| !content.trim().is_empty())
                .ok_or_else(|| anyhow!("Ollama generate response did not contain response text"))
        }
    }
}

fn extract_api_error(body: &str) -> Option<String> {
    serde_json::from_str::<ApiErrorResponse>(body)
        .ok()
        .map(|response| response.error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_is_extracted_from_json() {
        let error = extract_api_error(r#"{"error":"model 'llama3.1' not found"}"#);
        assert_eq!(error.as_deref(), Some("model 'llama3.1' not found"));
    }

    #[test]
    fn chat_message_builders_use_expected_roles() {
        assert_eq!(ChatMessage::user("hello").role, "user");
        assert_eq!(ChatMessage::assistant("hi").role, "assistant");
    }
}
