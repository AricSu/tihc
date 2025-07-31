use axum::{Extension, Json, extract::Path};
use core::platform::ServiceRegistry;
use core::platform::command_registry::CommandRegistry;
use serde_json::Value;
use std::sync::Arc;

/// HTTP API handler: /api/slowlog/scan-files
pub async fn handle_scan_files(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    tracing::info!(target: "slowlog_api", "[handle_scan_files] payload: {:?}", payload);
    // 兼容前端传递 { params: { logDir, pattern } } 和顶层 logDir/pattern
    let (dir, pattern) = if let Some(params) = payload.get("params") {
        let dir = params.get("logDir").and_then(|v| v.as_str()).unwrap_or("");
        let pattern = params.get("pattern").and_then(|v| v.as_str()).unwrap_or("*.log");
        (dir, pattern)
    } else {
        let dir = payload.get("logDir").and_then(|v| v.as_str()).unwrap_or("");
        let pattern = payload.get("pattern").and_then(|v| v.as_str()).unwrap_or("*.log");
        (dir, pattern)
    };
    tracing::info!(target: "slowlog_api", "[handle_scan_files] dir: {}, pattern: {}", dir, pattern);
    if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        let args = vec![dir.to_string(), pattern.to_string()];
        tracing::info!(target: "slowlog_api", "[handle_scan_files] exec args: {:?}", args);
        match cmd_reg.execute("slowlog-scan", &args) {
            Ok(res) => {
                tracing::info!(target: "slowlog_api", "[handle_scan_files] exec result: {:?}", res);
                Json(serde_json::json!({"status": "ok", "result": res}))
            }
            Err(e) => {
                tracing::error!(target: "slowlog_api", "[handle_scan_files] exec error: {}", e);
                let (code, reason) = if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                    match io_err.kind() {
                        std::io::ErrorKind::NotFound => (404, "not_found"),
                        std::io::ErrorKind::PermissionDenied => (403, "permission"),
                        _ => (500, "fs_error"),
                    }
                } else {
                    (500, "internal")
                };
                Json(serde_json::json!({
                    "error": e.to_string(),
                    "code": code,
                    "reason": reason
                }))
            }
        }
    } else {
        tracing::error!(target: "slowlog_api", "[handle_scan_files] command registry not found");
        Json(serde_json::json!({
            "error": "command registry not found",
            "code": 500
        }))
    }
}

/// Handles processing slowlog files.
pub async fn handle_process_slowlog(
    Extension(registry): Extension<Arc<ServiceRegistry>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    tracing::info!(target: "slowlog_api", "[handle_process_slowlog] payload: {:?}", payload);
    // 1. 解析 connectionId 和 files
    let connection_id = payload.get("connectionId").and_then(|v| v.as_u64()).unwrap_or(0);

    // 2. 通过 sql editor 命令分发获取 connection 信息
    tracing::info!(target: "slowlog_api", "[handle_process_slowlog] try get connection info for id: {}", connection_id);
    let conn_args = vec![connection_id.to_string()];
    let mut connection_info = if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        match cmd_reg.execute("editor-connections-get", &conn_args) {
            Ok(res) => {
                tracing::info!(target: "slowlog_api", "[handle_process_slowlog] got connection info: {:?}", res);
                res
            },
            Err(e) => {
                tracing::error!(target: "slowlog_api", "[handle_process_slowlog] connection not found: {}", e);
                return Json(serde_json::json!({"error": format!("connection not found: {}", e)}));
            }
        }
    } else {
        tracing::error!(target: "slowlog_api", "[handle_process_slowlog] service registry not found");
        return Json(serde_json::json!({"error": "service registry not found"}));
    };
    // 强制替换 database 字段为 "tihc"
    if let Some(obj) = connection_info.as_object_mut() {
        obj.insert("database".to_string(), serde_json::Value::String("tihc".to_string()));
    }

    // 3. 按插件 handler 期望顺序分发参数：log_dir, pattern, conn_json
    let log_dir = payload.get("logDir").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let pattern = payload.get("pattern").and_then(|v| v.as_str()).unwrap_or("*.log").to_string();
    let conn_json = serde_json::to_string(&connection_info).unwrap_or_default();
    let args = vec![log_dir, pattern, conn_json];
    tracing::info!(target: "slowlog_api", "[handle_process_slowlog] dispatch slowlog-import to plugin, args: {:?}", args);
    if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
        match cmd_reg.execute("slowlog-import", &args) {
            Ok(res) => {
                tracing::info!(target: "slowlog_api", "[handle_process_slowlog] plugin result: {:?}", res);
                Json(serde_json::json!({"status": "success", "result": res}))
            },
            Err(e) => {
                tracing::error!(target: "slowlog_api", "[handle_process_slowlog] plugin error: {}", e);
                Json(serde_json::json!({"error": e.to_string()}))
            }
        }
    } else {
        tracing::error!(target: "slowlog_api", "[handle_process_slowlog] service registry not found (plugin)");
        Json(serde_json::json!({"error": "service registry not found"}))
    }
}

/// Handles getting progress of a slowlog job.
pub async fn handle_get_progress(Path(job_id): Path<String>) -> Json<Value> {
    // TODO: 查询进度
    Json(serde_json::json!({"job_id": job_id, "progress": 0.5}))
}
