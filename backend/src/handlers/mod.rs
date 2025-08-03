pub mod slowlog;
pub mod editor_sql;

/// TODO: Remove this stub when real implementation is ready
use axum::Json;
use serde_json::Value;
pub async fn handle_issue_info(Json(_payload): Json<Value>) -> Json<Value> {
    Json(serde_json::json!({"status": "ok", "message": "stub"}))
}
