use thiserror::Error;

#[derive(Debug, Error)]
pub enum AutoflowError {
    #[error("http request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("request timeout")]
    Timeout,

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("invalid response format: {0}")]
    InvalidResponse(String),

    #[error("session not found: {0}")]
    SessionNotFound(String),

    #[error("stream error: {0}")]
    StreamError(String),

    #[error("authentication failed")]
    AuthenticationFailed,
}
