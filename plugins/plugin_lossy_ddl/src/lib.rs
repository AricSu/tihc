//! DDL Lossy Operation Detection Library using TiDB
//! 
//! This library provides precise analysis of DDL (Data Definition Language) 
//! statements using TiDB's schema tracker to detect lossy operations.
//!
//! # Features
//! 
//! - TiDB-powered precise lossy DDL detection
//! - Binary risk assessment (Safe or High only)
//! - No assumptions - only confirmed analysis results
//! - Collation support configuration
//!
//! # Quick Start
//!
//! ```rust
//! use plugin_lossy_ddl::{precheck_sql_with_collation, RiskLevel, LossyStatus};
//!
//! let sql = "CREATE DATABASE testdb; CREATE TABLE testdb.users (id INT, name VARCHAR(50)); ALTER TABLE testdb.users MODIFY COLUMN name VARCHAR(10);";
//! let result = precheck_sql_with_collation(sql, true);
//! 
//! // This is a lossy operation (VARCHAR 50 -> 10 may truncate data)
//! assert_eq!(result.lossy_status, LossyStatus::Lossy);
//! assert_eq!(result.risk_level, RiskLevel::High);
//! ```

#[cfg(feature = "tidb-engine")]
use std::ffi::CString;
#[cfg(feature = "tidb-engine")]
use std::os::raw::c_char;

pub(crate) mod error;
pub(crate) mod types;
pub mod plugin;

// Re-exports for convenience - only keep what's needed for plugin interface
pub use types::{AnalysisResult, RiskLevel, LossyStatus};
pub use plugin::{LossyDDLPlugin, DDLAnalysisHandler};
pub use error::DDLError;

#[cfg(feature = "tidb-engine")]
extern "C" {
    #[link_name = "precheck_sql_with_collation"]
    fn tidb_precheck_sql_with_collation(sql_ptr: *const c_char, collation_enabled: i32) -> i32;
}

/// Create an error result from DDLError
fn create_error_result_from_error(error: error::DDLError) -> AnalysisResult {
    AnalysisResult {
        lossy_status: types::LossyStatus::Unknown,
        risk_level: RiskLevel::High,
        warnings: vec![format!("❌ {}", error)],
        error: Some(error.to_string()),
    }
}

/// Create a success result
fn create_success_result(lossy_status: types::LossyStatus, message: &str) -> AnalysisResult {
    AnalysisResult {
        lossy_status: lossy_status.clone(),
        risk_level: if lossy_status.is_risky() { RiskLevel::High } else { RiskLevel::Safe },
        warnings: vec![message.to_string()],
        error: None,
    }
}

/// Validate SQL input
fn validate_sql(sql: &str) -> Result<(), error::DDLError> {
    if sql.trim().is_empty() {
        return Err(error::DDLError::InvalidInput("SQL statement cannot be empty".to_string()));
    }
    
    if sql.contains('\0') {
        return Err(error::DDLError::InvalidInput("SQL contains null bytes".to_string()));
    }
    
    Ok(())
}

#[cfg(feature = "tidb-engine")]
/// Analyze SQL using TiDB engine
fn analyze_with_tidb(sql: &str, collation_enabled: bool) -> Result<AnalysisResult, error::DDLError> {
    let c_sql = CString::new(sql)
        .map_err(|_| error::DDLError::InvalidInput("Failed to convert SQL to C string".to_string()))?;
    
    let result = unsafe {
        tidb_precheck_sql_with_collation(c_sql.as_ptr(), if collation_enabled { 1 } else { 0 })
    };
    
    match result {
        0 => Ok(create_success_result(types::LossyStatus::Safe, "✅ Safe operation")),
        1 => Ok(create_success_result(types::LossyStatus::Lossy, "⚠️ Lossy operation detected")),
        -1 => Err(error::DDLError::TiDBError("TiDB analysis failed".to_string())),
        _ => Err(error::DDLError::TiDBError(format!("Unknown TiDB result: {}", result))),
    }
}

/// Analyze SQL with collation setting - the main API function
/// 
/// # Arguments
/// * `sql` - The SQL statement to analyze
/// * `collation_enabled` - Whether to enable new collation in TiDB
/// 
/// # Returns
/// * `AnalysisResult` - Complete analysis result
pub fn precheck_sql_with_collation(sql: &str, collation_enabled: bool) -> AnalysisResult {
    // Validate input
    if let Err(error) = validate_sql(sql) {
        return create_error_result_from_error(error);
    }

    #[cfg(feature = "tidb-engine")]
    {
        match analyze_with_tidb(sql, collation_enabled) {
            Ok(result) => result,
            Err(error) => create_error_result_from_error(error),
        }
    }
    
    #[cfg(not(feature = "tidb-engine"))]
    {
        let _ = collation_enabled; // 避免未使用警告
        create_success_result(types::LossyStatus::Unknown, "ℹ️ TiDB engine not available")
    }
}