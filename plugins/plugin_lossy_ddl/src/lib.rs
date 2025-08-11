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
//! use plugin_lossy_ddl::{precheck_sql_with_collation, RiskLevel};
//!
//! let result = precheck_sql_with_collation("DROP TABLE users", true);
//! 
//! assert!(result.is_lossy);
//! assert_eq!(result.risk_level, RiskLevel::High);
//! ```

pub mod error;
pub mod types;
pub mod analyzer;
pub mod plugin;

// Re-exports for convenience
pub use types::{AnalysisResult, PrecheckResult, RiskLevel};
pub use error::{DDLError, DDLResult};
pub use plugin::{LossyDDLPlugin, DDLAnalysisHandler};

/// Analyze SQL with collation setting - the only supported analysis function
/// 
/// # Arguments
/// * `sql` - The SQL statement to analyze
/// * `collation_enabled` - Whether to enable new collation in TiDB
/// 
/// # Returns
/// * `AnalysisResult` - Complete analysis result
pub fn precheck_sql_with_collation(sql: &str, collation_enabled: bool) -> AnalysisResult {
    analyzer::analyze_sql(sql, collation_enabled)
}