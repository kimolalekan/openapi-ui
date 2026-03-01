//! Error types for the openapi-ui crate.

use thiserror::Error;

/// Errors that can occur when generating OpenAPI documentation UI.
#[derive(Error, Debug)]
pub enum UIError {
    /// Failed to parse the OpenAPI specification.
    #[error("Failed to parse OpenAPI specification: {0}")]
    ParseError(String),

    /// An I/O error occurred (e.g., writing to a file).
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Failed to serialize or deserialize JSON.
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Invalid configuration was provided.
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

/// A specialized `Result` type for openapi-ui operations.
pub type Result<T> = std::result::Result<T, UIError>;
