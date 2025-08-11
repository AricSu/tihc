//! DDL Lossy Operation Analyzer

use crate::{
    types::{AnalysisResult, RiskLevel},
    error::DDLError,
};

#[cfg(feature = "tidb-engine")]
use std::ffi::CString;
#[cfg(feature = "tidb-engine")]
use std::os::raw::c_char;

#[cfg(feature = "tidb-engine")]
extern "C" {
    fn precheck_sql_with_collation(sql_ptr: *const c_char, collation_enabled: i32) -> i32;
}

/// Analyze SQL statement for lossy operations
/// 
/// # Arguments
/// * `sql` - The SQL statement to analyze
/// * `collation_enabled` - Whether collation is enabled
/// 
/// # Returns
/// * `AnalysisResult` - Complete analysis result
pub(crate) fn analyze_sql(
    sql: &str, 
    #[cfg(feature = "tidb-engine")] collation_enabled: bool,
    #[cfg(not(feature = "tidb-engine"))] _collation_enabled: bool,
) -> AnalysisResult {
    // Input validation
    if let Err(error) = validate_input(sql) {
        return create_validation_error_result(error);
    }
    
    // Only support TiDB analysis
    #[cfg(feature = "tidb-engine")]
    {
        match analyze_with_tidb(sql, collation_enabled) {
            Ok(result) => result,
            Err(error) => create_tidb_error_result(error),
        }
    }
    
    #[cfg(not(feature = "tidb-engine"))]
    {
        AnalysisResult {
            is_lossy: true,
            risk_level: RiskLevel::High,
            warnings: vec![
                "⚠️ TiDB engine required for DDL analysis".to_string()
            ],
            error: Some("TiDB engine not available".to_string()),
            analyzed_patterns: vec![],
        }
    }
}
/// Validate input SQL
fn validate_input(sql: &str) -> Result<(), DDLError> {
    if sql.trim().is_empty() {
        return Err(DDLError::InvalidInput("SQL statement cannot be empty".to_string()));
    }
    
    if sql.contains('\0') {
        return Err(DDLError::InvalidInput("SQL statement contains null bytes".to_string()));
    }
    
    Ok(())
}

#[cfg(feature = "tidb-engine")]
/// Analyze SQL using TiDB
fn analyze_with_tidb(sql: &str, collation_enabled: bool) -> Result<AnalysisResult, DDLError> {
    let c_sql = CString::new(sql).map_err(|_| {
        DDLError::InvalidInput("Failed to convert SQL to C string".to_string())
    })?;
    
    let result = unsafe {
        precheck_sql_with_collation(
            c_sql.as_ptr(), 
            if collation_enabled { 1 } else { 0 }
        )
    };
    
    match result {
        0 => Ok(AnalysisResult {
            is_lossy: false,
            risk_level: RiskLevel::Safe,
            warnings: vec!["✅ TiDB confirmed this operation is safe".to_string()],
            error: None,
            analyzed_patterns: vec!["TiDB Schema Tracker".to_string()],
        }),
        1 => Ok(AnalysisResult {
            is_lossy: true,
            risk_level: RiskLevel::High,
            warnings: vec![
                "⚠️ TiDB detected this operation will cause data loss".to_string(),
                "💡 Create backup before execution".to_string(),
            ],
            error: None,
            analyzed_patterns: vec!["TiDB Schema Tracker".to_string()],
        }),
        -1 => Err(DDLError::TiDBError("TiDB analysis failed".to_string())),
        _ => Err(DDLError::TiDBError(format!("Unknown TiDB result: {}", result))),
    }
}

/// Create result for input validation errors
fn create_validation_error_result(error: DDLError) -> AnalysisResult {
    AnalysisResult {
        is_lossy: true, // Treat validation errors as high risk
        risk_level: RiskLevel::High,
        warnings: vec![format!("❌ Input validation failed: {}", error)],
        error: Some(error.to_string()),
        analyzed_patterns: vec![],
    }
}

#[cfg(feature = "tidb-engine")]
/// Create result for TiDB analysis errors
fn create_tidb_error_result(error: DDLError) -> AnalysisResult {
    AnalysisResult {
        is_lossy: true, // Treat analysis failures as high risk
        risk_level: RiskLevel::High,
        warnings: vec![
            "⚠️ TiDB analysis failed - treating as high risk".to_string(),
            "💡 Manual review required".to_string(),
        ],
        error: Some(format!("TiDB analysis failed: {}", error)),
        analyzed_patterns: vec![],
    }
}
