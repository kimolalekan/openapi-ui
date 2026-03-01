use thiserror::Error;

#[derive(Error, Debug)]
pub enum UIError {
    #[error("Failed to parse OpenAPI specification: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, UIError>;
