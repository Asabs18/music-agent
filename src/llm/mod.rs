pub mod ollama;

use crate::error::Result;
use async_trait::async_trait;

/// Abstract LLM client trait - allows swapping providers easily
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Send a prompt to the LLM and get a response
    async fn generate(&self, prompt: &str) -> Result<String>;

    /// Get the name of the LLM provider
    fn provider_name(&self) -> &str;
}
