//! Error types for DDL analysis operations

use thiserror::Error;

/// Errors that can occur during DDL analysis
#[derive(Debug, Error, Clone)]
pub(crate) enum DDLError {
    /// Input validation failed
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// TiDB engine error
    #[error("TiDB engine error: {0}")]
    TiDBError(String),
}


