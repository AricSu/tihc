// Slowlog HTTP Controllers

use crate::application::slowlog::SlowlogApplicationService;
use crate::interface::http::responses::ApiResponse;
use axum::{extract::Path, routing::post, Json, Router};
use serde_json::Value;
use std::sync::Arc;

pub struct SlowlogController;

impl SlowlogController {
    pub fn routes() -> Router<SlowlogAppState> {
        Router::new()
            .route("/api/slowlog/scan-files", post(scan_files_handler))
            .route("/api/slowlog/process", post(process_slowlog_handler))
            .route("/api/slowlog/progress/{job_id}", post(get_progress_handler))
    }
}

#[derive(Clone)]
pub struct SlowlogAppState {
    pub slowlog_service: Arc<dyn SlowlogApplicationService>,
}

impl SlowlogAppState {
    pub fn new(slowlog_service: Arc<dyn SlowlogApplicationService>) -> Self {
        Self { slowlog_service }
    }
}

/// HTTP API handler: /api/slowlog/scan-files
async fn scan_files_handler(
    axum::extract::State(state): axum::extract::State<SlowlogAppState>,
    Json(payload): Json<Value>,
) -> Json<ApiResponse<Value>> {
    tracing::info!(target: "slowlog_api", "[scan_files] payload: {:?}", payload);

    // 兼容前端传递 { params: { logDir, pattern } } 和顶层 logDir/pattern
    let (dir, pattern) = if let Some(params) = payload.get("params") {
        let dir = params.get("logDir").and_then(|v| v.as_str()).unwrap_or("");
        let pattern = params
            .get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("*.log");
        (dir, pattern)
    } else {
        let dir = payload.get("logDir").and_then(|v| v.as_str()).unwrap_or("");
        let pattern = payload
            .get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("*.log");
        (dir, pattern)
    };

    match state.slowlog_service.scan_files(dir, pattern).await {
        Ok(result) => Json(ApiResponse::success(result)),
        Err(e) => Json(ApiResponse::error(&e.to_string(), 500)),
    }
}

/// Handles processing slowlog files.
async fn process_slowlog_handler(
    axum::extract::State(state): axum::extract::State<SlowlogAppState>,
    Json(payload): Json<Value>,
) -> Json<ApiResponse<Value>> {
    tracing::info!(target: "slowlog_api", "[process_slowlog] payload: {:?}", payload);

    let connection_id = payload
        .get("connectionId")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let log_dir = payload.get("logDir").and_then(|v| v.as_str()).unwrap_or("");
    let pattern = payload
        .get("pattern")
        .and_then(|v| v.as_str())
        .unwrap_or("*.log");

    match state
        .slowlog_service
        .process_slowlog(connection_id, log_dir, pattern)
        .await
    {
        Ok(result) => Json(ApiResponse::success(result)),
        Err(e) => Json(ApiResponse::error(&e.to_string(), 500)),
    }
}

/// Handles getting progress of a slowlog job.
async fn get_progress_handler(
    axum::extract::State(state): axum::extract::State<SlowlogAppState>,
    Path(job_id): Path<String>,
) -> Json<ApiResponse<Value>> {
    match state.slowlog_service.get_progress(&job_id).await {
        Ok(result) => Json(ApiResponse::success(result)),
        Err(e) => Json(ApiResponse::error(&e.to_string(), 500)),
    }
}
