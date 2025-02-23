use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("No mock response configured for this request")]
    NoMockResponse,

    #[error("Queue is full")]
    QueueFull,

    #[error("Request timeout")]
    Timeout,

    #[error("No available endpoints")]
    NoEndpoints,

    #[error("Invalid configuration: {0}")]
    Config(String),
} 