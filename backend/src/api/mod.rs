//! API module: registers all HTTP routes and middleware.
pub mod database;
pub mod table;
pub mod sql;
pub mod notifications;

// 其他原有模块可保留
pub mod healthz;
pub mod slowlog;
use crate::middleware;
use axum::{Extension, Router};
use std::sync::Arc;
use core::platform::ServiceRegistry;

/// Returns the main API router with all routes and essential middleware.
pub fn create_router(registry: Arc<ServiceRegistry>) -> Router {
    Router::new()
        .merge(database::routes(registry.clone()))
        .merge(table::routes())
        .merge(sql::routes())
        .merge(notifications::routes())
        .merge(healthz::routes())
        .merge(slowlog::routes())
        .layer(Extension(registry))
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer())
}
