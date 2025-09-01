pub mod ddl_precheck;
pub mod editor_database;
pub mod editor_notifications;
pub mod editor_sql;
pub mod editor_table;
pub mod healthz;
pub mod lang;
pub mod slowlog;
pub mod static_routes;
use super::middleware;
use axum::{Extension, Router};
use microkernel::platform::ServiceRegistry;
use std::sync::Arc;

pub fn create_router(registry: Arc<std::sync::Mutex<ServiceRegistry>>) -> Router {
    let api_routes = Router::new()
        .merge(editor_database::routes(registry.clone()))
        .merge(editor_table::routes())
        .merge(editor_sql::routes())
        .merge(editor_notifications::routes())
        .merge(healthz::routes())
        .merge(slowlog::routes())
        .merge(ddl_precheck::routes())
        .merge(static_routes::static_routes())
        .merge(lang::lang_router())
        .fallback(static_routes::not_found_handler)
    .layer(Extension(registry));
    api_routes
}
