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

use tracing;
pub(crate) mod error;
pub mod plugin;
pub(crate) mod types;

// Re-exports for convenience - only keep what's needed for plugin interface
pub use error::DDLError;
pub use plugin::{DDLAnalysisHandler, LossyDDLPlugin};
pub use types::{AnalysisResult, LossyStatus, RiskLevel};

#[cfg(feature = "tidb-engine")]
use std::ffi::{CStr, CString};
#[cfg(feature = "tidb-engine")]
use std::os::raw::c_char;

#[cfg(feature = "tidb-engine")]
extern "C" {
    #[link_name = "precheck_sql_c"]
    fn tidb_precheck_sql_c(
        sql_ptr: *const c_char,
        collation_enabled: i32,
        verbose: i32,
        error_msg: *mut *mut c_char,
    ) -> i32;
    #[link_name = "free_error_message"]
    fn tidb_free_error_message(msg: *mut c_char);
}

/// Create an error result from DDLError
fn create_error_result_from_error(error: error::DDLError) -> AnalysisResult {
    AnalysisResult {
        lossy_status: types::LossyStatus::Unknown,
        risk_level: RiskLevel::High,
        warnings: vec![],
        error: Some(error.to_string()),
    }
}

/// Create a success result
fn create_success_result(lossy_status: types::LossyStatus, message: &str) -> AnalysisResult {
    AnalysisResult {
        lossy_status: lossy_status.clone(),
        risk_level: if lossy_status.is_risky() {
            RiskLevel::High
        } else {
            RiskLevel::Safe
        },
        warnings: vec![message.to_string()],
        error: None,
    }
}

/// Validate SQL input
fn validate_sql(sql: &str) -> Result<(), error::DDLError> {
    if sql.trim().is_empty() {
        return Err(error::DDLError::InvalidInput(
            "SQL statement cannot be empty".to_string(),
        ));
    }

    if sql.contains('\0') {
        return Err(error::DDLError::InvalidInput(
            "SQL contains null bytes".to_string(),
        ));
    }

    Ok(())
}

#[cfg(feature = "tidb-engine")]
/// Analyze SQL using TiDB engine with verbose output
fn analyze_with_tidb(
    sql: &str,
    collation_enabled: bool,
) -> Result<AnalysisResult, error::DDLError> {
    let c_sql = CString::new(sql).map_err(|_| {
        error::DDLError::InvalidInput("Failed to convert SQL to C string".to_string())
    })?;

    let mut error_msg: *mut c_char = std::ptr::null_mut();

    // 调用 Go 的 precheck_sql_c 函数
    let status = unsafe {
        tidb_precheck_sql_c(
            c_sql.as_ptr(),
            if collation_enabled { 1 } else { 0 },
            0, // verbose enabled
            &mut error_msg,
        )
    };

    // 始终释放 error_msg 内存，避免泄漏
    let result = match status {
        0 => {
            tracing::info!("✅ DDL analysis completed: Safe operation");
            Ok(create_success_result(
                types::LossyStatus::Safe,
                "✅ Safe operation",
            ))
        }
        1 => {
            tracing::warn!("⚠️ DDL analysis completed: Lossy operation detected");
            Ok(create_success_result(
                types::LossyStatus::Lossy,
                "⚠️ Lossy operation detected",
            ))
        }
        -1 => {
            // 获取详细错误信息
            let error_string = if error_msg.is_null() {
                "Unknown TiDB error".to_string()
            } else {
                unsafe {
                    let c_str = CStr::from_ptr(error_msg);
                    let error_str = c_str.to_string_lossy().to_string();
                    error_str
                }
            };
            tracing::error!("❌ DDL analysis failed: {}", error_string);
            Err(error::DDLError::TiDBError(error_string))
        }
        _ => {
            tracing::error!("❌ DDL analysis failed: Unknown status code {}", status);
            Err(error::DDLError::TiDBError(format!(
                "Unknown status code: {}",
                status
            )))
        }
    };
    if !error_msg.is_null() {
        unsafe {
            tidb_free_error_message(error_msg);
        }
    }
    result
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

    // 先全部转小写检测 RENAME COLUMN
    let mut rename_column_warnings = vec![];
    for line in sql.split(';') {
        let line = line.trim();
        let line_lower = line.to_lowercase();
        // 只检测小写
        let re = regex::Regex::new(
            r"^alter\s+table\s+([\w.]+)\s+rename\s+column\s+([\w`]+)\s+to\s+([\w`]+)",
        )
        .unwrap();
        if let Some(_caps) = re.captures(&line_lower) {
            // 由于原始表名/字段名可能有大小写/反引号，需从原始 line 再提取
            let orig_re = regex::Regex::new(
                r"(?i)^ALTER\s+TABLE\s+([\w.]+)\s+RENAME\s+COLUMN\s+([\w`]+)\s+TO\s+([\w`]+)",
            )
            .unwrap();
            if let Some(orig_caps) = orig_re.captures(line) {
                let table = orig_caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let old = orig_caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let new = orig_caps.get(3).map(|m| m.as_str()).unwrap_or("");
                let warn1 =
                    "Detected unsupported RENAME COLUMN syntax in TiHC DDL Precheck.".to_string();
                let warn2 = format!(
                    "Recommended: ALTER TABLE {table} CHANGE {old} {new} <column_type>; ",
                    table = table,
                    old = old,
                    new = new
                );
                rename_column_warnings.push(warn1);
                rename_column_warnings.push(warn2);
            }
        }
    }

    // 命中则直接返回 warning，不再调用 analyze_with_tidb
    if !rename_column_warnings.is_empty() {
        return AnalysisResult {
            lossy_status: types::LossyStatus::Unknown,
            risk_level: RiskLevel::High,
            warnings: rename_column_warnings,
            error: None,
        };
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
// 依赖 regex crate
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rename_column_detect() {
        let sql = "ALTER TABLE test.aric RENAME COLUMN name TO name1;";
        let result = precheck_sql_with_collation(sql, false);
        assert!(result.warnings.iter().any(|w| w.contains("RENAME COLUMN")));
    }
}
