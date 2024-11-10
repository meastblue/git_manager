use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("API error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
