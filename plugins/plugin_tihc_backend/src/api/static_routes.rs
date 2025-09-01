use crate::handlers::static_files;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

/// Creates router for static file serving and SPA routing
pub fn static_routes() -> Router {
       Router::new()
	       // Serve frontend index.html at root
	       .route("/", get(static_files::index_handler))
	       // Serve static files under /assets/*
			   .route("/assets/{*file}", get(static_files::static_handler))
	       // Catch-all route for SPA routing (history fallback)
	       .fallback(static_files::static_handler)
}

pub async fn not_found_handler() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "404 Not Found")
}
