use crate::handlers::static_files;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

/// Creates router for static file serving and SPA routing
pub fn static_routes() -> Router {
    Router::new()
        // Serve frontend index.html at root
        .route("/", get(static_files::index_handler))
        // Catch-all route for static files and SPA routing
        .fallback(static_files::static_handler)
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
