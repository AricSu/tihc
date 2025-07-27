//! API module: registers all HTTP routes and middleware.
pub mod healthz;
pub mod slowlog;
use crate::middleware;
use axum::{Extension, Router};
use std::sync::Arc;

/// Returns the main API router with only slowlog routes and essential middleware.
pub fn create_router(db: Arc<()>) -> Router {
    Router::new()
        .merge(healthz::routes())
        .merge(slowlog::routes())
        .layer(Extension(db))
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer())
}
