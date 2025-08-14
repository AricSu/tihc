use crate::handlers::ddl_precheck::handle_ddl_precheck;
use axum::{Router, routing::post};
use plugin_lossy_ddl::LossyStatus;
use serde::{Deserialize, Serialize};

/// DDL 预检查请求体
#[derive(Deserialize)]
pub struct DDLPrecheckRequest {
    /// 待分析的 SQL 语句
    pub sql: String,
    /// 是否启用新的排序规则
    #[serde(default = "default_collation_enabled")]
    pub collation_enabled: bool,
}

fn default_collation_enabled() -> bool {
    true
}

/// DDL 预检查响应
#[derive(Serialize, Default, Debug, Clone)]
pub struct DDLPrecheckResponse {
    /// 是否为有损操作
    pub lossy_status: LossyStatus,
    /// 风险级别（Safe 或 High）
    pub risk_level: String,
    /// 检测到的问题描述
    pub issues: Vec<String>,
    /// 错误信息（如果有）
    pub error: Option<String>,
    /// 建议操作
    pub recommendations: Vec<String>,
}

/// DDL 风险级别枚举
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum RiskLevel {
    Safe,
    High,
}

impl From<plugin_lossy_ddl::RiskLevel> for RiskLevel {
    fn from(risk: plugin_lossy_ddl::RiskLevel) -> Self {
        match risk {
            plugin_lossy_ddl::RiskLevel::Safe => RiskLevel::Safe,
            plugin_lossy_ddl::RiskLevel::High => RiskLevel::High,
        }
    }
}

impl From<RiskLevel> for String {
    fn from(risk: RiskLevel) -> Self {
        match risk {
            RiskLevel::Safe => "Safe".to_string(),
            RiskLevel::High => "High".to_string(),
        }
    }
}

/// 创建 DDL 预检查相关的路由
pub fn routes() -> Router {
    Router::new().route("/ddl/precheck", post(handle_ddl_precheck))
}
