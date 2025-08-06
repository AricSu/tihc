//! Slowlog API router.
use crate::handlers::slowlog::{handle_get_progress, handle_process_slowlog, handle_scan_files};
use axum::{
    Router,
    routing::{get, post},
};

/// Returns a router with all slowlog endpoints.
pub fn routes() -> Router {
    Router::new()
        .route("/slowlog/scan-files", post(handle_scan_files))
        .route("/slowlog/process", post(handle_process_slowlog))
        .route("/slowlog/progress/{job_id}", get(handle_get_progress))
}
