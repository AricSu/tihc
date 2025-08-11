use axum::{http::StatusCode, Json};
use tracing::{info, warn};

use crate::api::ddl_precheck::{DDLPrecheckRequest, DDLPrecheckResponse, RiskLevel};

/// 处理 DDL 预检查请求
pub async fn handle_ddl_precheck(
    Json(request): Json<DDLPrecheckRequest>,
) -> Result<Json<DDLPrecheckResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("Received DDL precheck request for SQL: {}", request.sql);

    // 调用 lossy DDL 检测插件
    let analysis_result = plugin_lossy_ddl::precheck_sql_with_collation(
        &request.sql,
        request.collation_enabled,
    );

    // 处理分析结果 - 直接使用插件返回的信息，不做额外判断
    let response = if let Some(error_msg) = analysis_result.error {
        warn!("DDL analysis error: {}", error_msg);
        DDLPrecheckResponse {
            is_lossy: analysis_result.is_lossy,
            risk_level: RiskLevel::from(analysis_result.risk_level).into(),
            issues: analysis_result.warnings,
            error: Some(error_msg),
            recommendations: vec![], // 错误时暂无建议
        }
    } else {
        let risk_level: RiskLevel = analysis_result.risk_level.into();
        
        DDLPrecheckResponse {
            is_lossy: analysis_result.is_lossy,
            risk_level: risk_level.into(),
            issues: analysis_result.warnings,
            error: None,
            recommendations: analysis_result.analyzed_patterns, // 使用分析模式作为建议信息
        }
    };

    info!(
        "DDL precheck completed - is_lossy: {}, risk_level: {}",
        response.is_lossy, response.risk_level
    );

    Ok(Json(response))
}
