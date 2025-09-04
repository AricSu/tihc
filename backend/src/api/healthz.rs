//! Health check API handler.
use axum::{Router, routing::get};

/// Returns a router with the /healthz endpoint.
pub fn routes() -> Router {
    Router::new().route("/healthz", get(healthz_handler))
}

/// Health check handler.
async fn healthz_handler() -> &'static str {
    "ok"
}
