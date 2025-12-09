use axum::routing::get;
use axum::Router;

pub fn static_dist_router() -> Router {
    Router::new()
        .route("/{*path}", get(static_handler))
        .route("/", get(index_handler))
}
use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, Uri, header};
use axum::response::IntoResponse;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../frontend/shared/dist"]
pub struct StaticFiles;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    let exists = StaticFiles::get(path).is_some();

    if exists {
        let content = StaticFiles::get(path).unwrap();
        let mime_type = mime_guess::from_path(path).first_or_octet_stream();
        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(Body::from(content.data.to_vec()))
            .unwrap();
    } else {
        tracing::warn!(target: "static_files", "File not found: {}, fallback to index.html", path);
        // For SPA routing, serve index.html for unknown routes
        let index_exists = StaticFiles::get("index.html").is_some();
        tracing::info!(target: "static_files", "index.html exists: {}", index_exists);
        if index_exists {
            let content = StaticFiles::get("index.html").unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str("text/html").unwrap(),
                )
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        } else {
            tracing::error!(target: "static_files", "index.html not found in embedded files!");
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap()
        }
    }
}

pub async fn index_handler() -> impl IntoResponse {
    static_handler("/".parse().unwrap()).await
}
