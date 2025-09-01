use axum::{Json, http::StatusCode};
use plugin_lossy_ddl::LossyStatus;
use tracing::{info, warn};
use crate::api::ddl_precheck::{DDLPrecheckRequest, DDLPrecheckResponse, RiskLevel};

fn generate_recommendations(lossy_status: &LossyStatus) -> Vec<String> {
	match lossy_status {
		LossyStatus::Lossy => vec![
			"Ensure no data is lost due to truncation, and run ANALYZE TABLE immediately after DDL to avoid statistics loss impacting SQL performance.".to_string()
		],
		LossyStatus::Safe => vec![],
		LossyStatus::Unknown => vec![
			"Please check your SQL syntax based on the provided hints.".to_string()
		],
	}
}

pub async fn handle_ddl_precheck(
	Json(request): Json<DDLPrecheckRequest>,
) -> Result<Json<DDLPrecheckResponse>, (StatusCode, Json<serde_json::Value>)> {
	info!("Received DDL precheck request for SQL: {}", request.sql);

	let analysis_result =
		plugin_lossy_ddl::precheck_sql_with_collation(&request.sql, request.collation_enabled);

	let response = if let Some(error_msg) = analysis_result.error {
		warn!("DDL analysis error: {}", error_msg);
		DDLPrecheckResponse {
			lossy_status: analysis_result.lossy_status.clone(),
			risk_level: RiskLevel::from(analysis_result.risk_level).into(),
			issues: analysis_result.warnings,
			error: Some(error_msg),
			recommendations: generate_recommendations(&analysis_result.lossy_status),
		}
	} else {
		let risk_level: RiskLevel = analysis_result.risk_level.into();
		DDLPrecheckResponse {
			lossy_status: analysis_result.lossy_status.clone(),
			risk_level: risk_level.into(),
			issues: analysis_result.warnings,
			error: None,
			recommendations: generate_recommendations(&analysis_result.lossy_status),
		}
	};

	info!(
		"DDL precheck completed - lossy_status: {}, risk_level: {}",
		response.lossy_status, response.risk_level
	);

	Ok(Json(response))
}
