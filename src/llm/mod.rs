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

/// Creates an LLM client based on environment configuration
/// Priority: ANTHROPIC_API_KEY > OPENAI_API_KEY > Ollama (default)
pub async fn create_llm_client() -> Result<Box<dyn LLMClient>> {
    // For MVP, we'll use Ollama by default
    // Later, we can check for API keys and use cloud providers
    
    // Check if Ollama is available
    let ollama = ollama::OllamaClient::new("http://localhost:11434");
    
    Ok(Box::new(ollama))
}
