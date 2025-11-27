// DDL Precheck Domain Layer
// 定义DDL预检查相关的核心业务逻辑和规则

use serde::{Deserialize, Serialize};
use std::fmt;

/// DDL 风险级别
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// 安全操作
    Safe,
    /// 高风险操作
    High,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RiskLevel::Safe => write!(f, "Safe"),
            RiskLevel::High => write!(f, "High"),
        }
    }
}

/// 有损状态枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LossyStatus {
    /// 安全操作，无数据丢失风险
    Safe,
    /// 有损操作，可能导致数据丢失
    Lossy,
    /// 未知状态，需要进一步检查
    Unknown,
}

impl fmt::Display for LossyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LossyStatus::Safe => write!(f, "Safe"),
            LossyStatus::Lossy => write!(f, "Lossy"),
            LossyStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// DDL 语句值对象
#[derive(Debug, Clone)]
pub struct DDLStatement {
    pub sql: String,
    pub collation_enabled: bool,
}

impl DDLStatement {
    /// 创建新的DDL语句对象
    pub fn new(sql: String, collation_enabled: bool) -> Result<Self, DDLValidationError> {
        if sql.trim().is_empty() {
            return Err(DDLValidationError::EmptySQL);
        }

        if sql.len() > 1_000_000 {
            // 1MB limit
            return Err(DDLValidationError::SQLTooLarge);
        }

        Ok(Self {
            sql: sql.trim().to_string(),
            collation_enabled,
        })
    }

    /// 检查是否为DDL语句
    pub fn is_ddl(&self) -> bool {
        let sql_upper = self.sql.to_uppercase();
        sql_upper.starts_with("CREATE")
            || sql_upper.starts_with("ALTER")
            || sql_upper.starts_with("DROP")
            || sql_upper.starts_with("TRUNCATE")
            || sql_upper.starts_with("RENAME")
    }

    /// 检查是否可能是高风险操作
    pub fn is_potentially_risky(&self) -> bool {
        let sql_upper = self.sql.to_uppercase();
        sql_upper.contains("DROP")
            || sql_upper.contains("TRUNCATE")
            || sql_upper.contains("ALTER") && sql_upper.contains("DROP COLUMN")
    }
}

/// DDL 分析结果聚合根
#[derive(Debug, Clone)]
pub struct DDLAnalysisResult {
    pub statement: DDLStatement,
    pub lossy_status: LossyStatus,
    pub risk_level: RiskLevel,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub error: Option<String>,
}

impl DDLAnalysisResult {
    /// 创建分析结果
    pub fn new(
        statement: DDLStatement,
        lossy_status: LossyStatus,
        risk_level: RiskLevel,
        issues: Vec<String>,
        error: Option<String>,
    ) -> Self {
        let recommendations = Self::generate_recommendations(&lossy_status, &issues);

        Self {
            statement,
            lossy_status,
            risk_level,
            issues,
            recommendations,
            error,
        }
    }

    /// 根据状态生成建议
    fn generate_recommendations(lossy_status: &LossyStatus, issues: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        match lossy_status {
            LossyStatus::Lossy => {
                recommendations.push(
                    "Ensure no data is lost due to truncation, and run ANALYZE TABLE immediately after DDL to avoid statistics loss impacting SQL performance.".to_string()
                );

                if issues.iter().any(|issue| issue.contains("DROP")) {
                    recommendations.push(
                        "Consider backing up affected data before executing DROP operations."
                            .to_string(),
                    );
                }
            }
            LossyStatus::Safe => {
                // 安全操作可能仍需要一些建议
                if issues.iter().any(|issue| issue.contains("performance")) {
                    recommendations.push(
                        "Consider executing during low-traffic periods for better performance."
                            .to_string(),
                    );
                }
            }
            LossyStatus::Unknown => {
                recommendations
                    .push("Please check your SQL syntax based on the provided hints.".to_string());
                recommendations.push("Review the SQL statement for potential syntax errors or unsupported operations.".to_string());
            }
        }

        recommendations
    }

    /// 检查是否有错误
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// 检查是否为高风险
    pub fn is_high_risk(&self) -> bool {
        self.risk_level == RiskLevel::High
    }
}

/// DDL验证错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DDLValidationError {
    EmptySQL,
    SQLTooLarge,
    InvalidSyntax(String),
    UnsupportedOperation(String),
}

impl fmt::Display for DDLValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DDLValidationError::EmptySQL => write!(f, "SQL statement cannot be empty"),
            DDLValidationError::SQLTooLarge => write!(f, "SQL statement is too large (max 1MB)"),
            DDLValidationError::InvalidSyntax(msg) => write!(f, "Invalid SQL syntax: {}", msg),
            DDLValidationError::UnsupportedOperation(op) => {
                write!(f, "Unsupported operation: {}", op)
            }
        }
    }
}

impl std::error::Error for DDLValidationError {}

/// DDL预检查领域服务
pub struct DDLPrecheckDomainService;

impl DDLPrecheckDomainService {
    /// 验证DDL语句
    pub fn validate_ddl_statement(
        sql: &str,
        collation_enabled: bool,
    ) -> Result<DDLStatement, DDLValidationError> {
        DDLStatement::new(sql.to_string(), collation_enabled)
    }

    /// 评估DDL风险级别
    pub fn assess_risk_level(statement: &DDLStatement, lossy_status: &LossyStatus) -> RiskLevel {
        match lossy_status {
            LossyStatus::Lossy => RiskLevel::High,
            LossyStatus::Safe => {
                if statement.is_potentially_risky() {
                    RiskLevel::High
                } else {
                    RiskLevel::Safe
                }
            }
            LossyStatus::Unknown => RiskLevel::High, // 未知状态视为高风险
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddl_statement_creation() {
        // 正常创建
        let stmt = DDLStatement::new("CREATE TABLE test (id INT)".to_string(), true).unwrap();
        assert_eq!(stmt.sql, "CREATE TABLE test (id INT)");
        assert!(stmt.collation_enabled);

        // 空SQL
        assert!(matches!(
            DDLStatement::new("".to_string(), true),
            Err(DDLValidationError::EmptySQL)
        ));

        // SQL过大
        let large_sql = "A".repeat(1_000_001);
        assert!(matches!(
            DDLStatement::new(large_sql, true),
            Err(DDLValidationError::SQLTooLarge)
        ));
    }

    #[test]
    fn test_ddl_statement_detection() {
        let create_stmt =
            DDLStatement::new("CREATE TABLE test (id INT)".to_string(), true).unwrap();
        assert!(create_stmt.is_ddl());

        let select_stmt = DDLStatement::new("SELECT * FROM test".to_string(), true).unwrap();
        assert!(!select_stmt.is_ddl());
    }

    #[test]
    fn test_risk_assessment() {
        let drop_stmt = DDLStatement::new("DROP TABLE test".to_string(), true).unwrap();
        assert!(drop_stmt.is_potentially_risky());

        let create_stmt =
            DDLStatement::new("CREATE TABLE test (id INT)".to_string(), true).unwrap();
        assert!(!create_stmt.is_potentially_risky());
    }

    #[test]
    fn test_analysis_result_recommendations() {
        let stmt = DDLStatement::new("DROP TABLE test".to_string(), true).unwrap();
        let result = DDLAnalysisResult::new(
            stmt,
            LossyStatus::Lossy,
            RiskLevel::High,
            vec!["DROP operation detected".to_string()],
            None,
        );

        assert!(!result.recommendations.is_empty());
        assert!(result.is_high_risk());
        assert!(!result.has_error());
    }

    #[test]
    fn test_domain_service_validation() {
        let result =
            DDLPrecheckDomainService::validate_ddl_statement("CREATE TABLE test (id INT)", true);
        assert!(result.is_ok());

        let empty_result = DDLPrecheckDomainService::validate_ddl_statement("", true);
        assert!(empty_result.is_err());
    }

    #[test]
    fn test_risk_level_assessment() {
        let stmt = DDLStatement::new("CREATE TABLE test (id INT)".to_string(), true).unwrap();

        let safe_risk = DDLPrecheckDomainService::assess_risk_level(&stmt, &LossyStatus::Safe);
        assert_eq!(safe_risk, RiskLevel::Safe);

        let lossy_risk = DDLPrecheckDomainService::assess_risk_level(&stmt, &LossyStatus::Lossy);
        assert_eq!(lossy_risk, RiskLevel::High);

        let unknown_risk =
            DDLPrecheckDomainService::assess_risk_level(&stmt, &LossyStatus::Unknown);
        assert_eq!(unknown_risk, RiskLevel::High);
    }
}
