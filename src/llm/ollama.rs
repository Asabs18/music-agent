use crate::error::{AgentError, Result};
use crate::llm::LLMClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            model: "llama3.2".to_string(), // Default model
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn generate(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);

        let request_body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                AgentError::LlmRequest(format!(
                    "Failed to connect to Ollama at {}. Is Ollama running? Error: {}",
                    self.base_url, e
                ))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AgentError::LlmRequest(format!(
                "Ollama request failed with status {}: {}",
                status, error_text
            )));
        }

        let ollama_response: OllamaResponse = response.json().await.map_err(|e| {
            AgentError::LlmResponse(format!("Failed to parse Ollama response: {}", e))
        })?;

        Ok(ollama_response.response)
    }

    fn provider_name(&self) -> &str {
        "Ollama"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_client_creation() {
        let client = OllamaClient::new("http://localhost:11434");
        assert_eq!(client.provider_name(), "Ollama");
        assert_eq!(client.model, "llama3.2");
    }

    #[test]
    fn test_ollama_client_with_model() {
        let client = OllamaClient::new("http://localhost:11434").with_model("mistral");
        assert_eq!(client.model, "mistral");
    }
}
