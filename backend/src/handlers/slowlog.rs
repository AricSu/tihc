use axum::{Extension, Json, extract::Path};
use std::sync::Arc;
use core::platform::ServiceRegistry;
use core::platform::command_registry::CommandRegistry;
use serde_json::Value;

/// HTTP API handler: /api/slowlog/scan-files
pub async fn handle_scan_files(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(payload): Json<Value>
) -> Json<Value> {
    let dir = payload.get("dir").and_then(|v| v.as_str()).unwrap_or("");
    let pattern = payload.get("pattern").and_then(|v| v.as_str()).unwrap_or("*.log");
    // 通过 CommandRegistry 调用 slowlog-scan handler
    if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        // You need to define args before using them, e.g.:
        let args = vec![dir.to_string(), pattern.to_string()];
        match cmd_reg.execute("slowlog-scan", &args) {
            Ok(_) => {
                // handler 内部已处理输出，这里可返回通用成功
                Json(serde_json::json!({"status": "ok"}))
            },
            Err(e) => Json(serde_json::json!({"error": e.to_string()})),
        }
    } else {
        Json(serde_json::json!({"error": "command registry not found"}))
    }
}

/// Handles processing slowlog files.
pub async fn handle_process_slowlog(Json(_payload): Json<Value>) -> Json<Value> {
    // TODO: 调用 slowlog 处理逻辑
    Json(serde_json::json!({"job_id": "123"}))
}

/// Handles getting progress of a slowlog job.
pub async fn handle_get_progress(Path(job_id): Path<String>) -> Json<Value> {
    // TODO: 查询进度
    Json(serde_json::json!({"job_id": job_id, "progress": 0.5}))
}