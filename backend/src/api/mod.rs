//! API module: registers all HTTP routes and middleware.
pub mod editor_database;
pub mod editor_notifications;
pub mod editor_sql;
pub mod editor_table;
pub mod healthz;
pub mod slowlog;
pub mod static_routes;
use crate::middleware;
use axum::{Extension, Router};
use microkernel::platform::ServiceRegistry;
use std::sync::Arc;

/// Returns the main API router with all routes and essential middleware.
pub fn create_router(registry: Arc<ServiceRegistry>) -> Router {
    // API routes with /api prefix
    let api_routes = Router::new()
        .merge(editor_database::routes(registry.clone()))
        .merge(editor_table::routes())
        .merge(editor_sql::routes())
        .merge(editor_notifications::routes())
        .merge(healthz::routes())
        .merge(slowlog::routes())
        .merge(static_routes::static_routes())
        .fallback(static_routes::not_found_handler)
        .layer(Extension(registry))
        .layer(middleware::cors_layer())
        .layer(middleware::trace_layer());

    api_routes
}
