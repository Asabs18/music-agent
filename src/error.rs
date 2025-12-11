use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Failed to read file: {0}")]
    FileRead(String),

    #[error("Failed to parse metadata: {0}")]
    MetadataParse(String),

    #[error("LLM request failed: {0}")]
    LlmRequest(String),

    #[error("LLM response invalid: {0}")]
    LlmResponse(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ID3 error: {0}")]
    Id3(#[from] id3::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AgentError>;
