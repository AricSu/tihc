// DDL Precheck Application Service
// 编排DDL预检查的业务流程，协调领域对象和外部服务

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{
    ddl_precheck::{DDLAnalysisResult, DDLPrecheckDomainService, LossyStatus},
    shared::{
        DomainResult,
        services::{DDLAnalysisService, ExternalAnalysisResult},
    },
};

/// DDL预检查请求DTO
#[derive(Debug, Deserialize)]
pub struct DDLPrecheckRequest {
    /// 待分析的SQL语句
    pub sql: String,
    /// 是否启用新的排序规则
    #[serde(default = "default_collation_enabled")]
    pub collation_enabled: bool,
}

fn default_collation_enabled() -> bool {
    true
}

// 移除了 PluginAnalysisResult，现在使用领域层的 DDLAnalysisResult

/// DDL预检查响应DTO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DDLPrecheckResponse {
    /// 有损状态
    pub lossy_status: String,
    /// 风险级别
    pub risk_level: String,
    /// 检测到的问题
    pub issues: Vec<String>,
    /// 建议操作
    pub recommendations: Vec<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 分析详情
    pub analysis_details: Option<String>,
}

impl From<DDLAnalysisResult> for DDLPrecheckResponse {
    fn from(result: DDLAnalysisResult) -> Self {
        Self {
            lossy_status: result.lossy_status.to_string(),
            risk_level: result.risk_level.to_string(),
            issues: result.issues,
            recommendations: result.recommendations,
            error: result.error,
            analysis_details: Some(format!(
                "DDL statement: {} (collation_enabled: {})",
                result.statement.sql.chars().take(100).collect::<String>(),
                result.statement.collation_enabled
            )),
        }
    }
}

/// DDL预检查应用服务trait
#[async_trait]
pub trait DDLPrecheckApplicationService: Send + Sync {
    async fn precheck_ddl(&self, request: DDLPrecheckRequest) -> DomainResult<DDLPrecheckResponse>;
}

/// DDL预检查应用服务实现
pub struct DDLPrecheckApplicationServiceImpl {
    analysis_service: Arc<dyn DDLAnalysisService>,
}

impl DDLPrecheckApplicationServiceImpl {
    pub fn new(analysis_service: Arc<dyn DDLAnalysisService>) -> Self {
        Self { analysis_service }
    }

    /// 将外部插件的LossyStatus字符串转换为领域的LossyStatus
    fn convert_lossy_status(plugin_status: &str) -> LossyStatus {
        match plugin_status {
            "Safe" => LossyStatus::Safe,
            "Lossy" => LossyStatus::Lossy,
            "Unknown" => LossyStatus::Unknown,
            _ => LossyStatus::Unknown, // 默认为未知
        }
    }

    /// 解析分析结果，提取问题列表
    fn extract_issues_from_analysis_result(
        analysis_result: &ExternalAnalysisResult,
    ) -> Vec<String> {
        let mut issues = analysis_result.warnings.clone();

        // 基于lossy status添加通用问题
        match analysis_result.lossy_status.as_str() {
            "Lossy" => {
                if issues.is_empty() {
                    issues.push("Operation may cause data loss or schema changes".to_string());
                }
            }
            "Unknown" => {
                issues.push("Unable to determine operation safety".to_string());
            }
            _ => {}
        }

        issues
    }
}

#[async_trait]
impl DDLPrecheckApplicationService for DDLPrecheckApplicationServiceImpl {
    async fn precheck_ddl(&self, request: DDLPrecheckRequest) -> DomainResult<DDLPrecheckResponse> {
        tracing::info!(target: "ddl_precheck_service", "Processing DDL precheck request for SQL: {}", request.sql);

        // 1. 使用领域服务验证DDL语句
        let ddl_statement = DDLPrecheckDomainService::validate_ddl_statement(
            &request.sql,
            request.collation_enabled,
        )
        .map_err(|e| crate::domain::shared::DomainError::ValidationError {
            message: e.to_string(),
        })?;

        // 2. 检查是否为DDL语句
        if !ddl_statement.is_ddl() {
            tracing::warn!(target: "ddl_precheck_service", "Non-DDL statement provided: {}", request.sql);
            return Ok(DDLPrecheckResponse {
                lossy_status: "Unknown".to_string(),
                risk_level: "Safe".to_string(),
                issues: vec!["Not a DDL statement".to_string()],
                recommendations: vec![
                    "Please provide a valid DDL statement (CREATE, ALTER, DROP, etc.)".to_string(),
                ],
                error: None,
                analysis_details: Some("Statement is not a DDL operation".to_string()),
            });
        }

        // 3. 通过注入的分析服务进行分析
        let analysis_result = self
            .analysis_service
            .analyze_ddl(&request.sql, request.collation_enabled)
            .await?;

        // 4. 转换外部结果为领域对象
        let lossy_status = Self::convert_lossy_status(&analysis_result.lossy_status);
        let risk_level = DDLPrecheckDomainService::assess_risk_level(&ddl_statement, &lossy_status);
        let issues = Self::extract_issues_from_analysis_result(&analysis_result);
        let error = analysis_result.error;

        // 5. 创建分析结果聚合根
        let analysis_result =
            DDLAnalysisResult::new(ddl_statement, lossy_status, risk_level, issues, error);

        tracing::info!(target: "ddl_precheck_service", 
                      "DDL precheck completed - lossy_status: {}, risk_level: {}", 
                      analysis_result.lossy_status, analysis_result.risk_level);

        // 6. 转换为响应DTO
        Ok(analysis_result.into())
    }
}
