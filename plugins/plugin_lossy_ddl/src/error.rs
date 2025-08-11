//! Error types for DDL analysis operations

use thiserror::Error;

/// Result type for DDL operations
pub type DDLResult<T> = Result<T, DDLError>;

/// Errors that can occur during DDL analysis
#[derive(Debug, Error, Clone)]
pub enum DDLError {
    /// Input validation failed
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// SQL parsing failed
    #[error("Failed to parse SQL: {0}")]
    ParseError(String),
    
    /// TiDB engine error
    #[error("TiDB engine error: {0}")]
    TiDBError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}


