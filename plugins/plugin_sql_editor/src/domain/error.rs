use common::CommonError;
use thiserror::Error;

/// Errors that can occur in the SQL Editor domain.
#[derive(Debug, Error)]
pub enum SqlEditorError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Other error: {0}")]
    Other(String),
    #[error("Infra error: {0}")]
    Infra(#[from] CommonError),
}
