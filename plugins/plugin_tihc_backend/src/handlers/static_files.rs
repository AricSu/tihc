
use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, Uri, header};
use axum::response::IntoResponse;
use rust_embed::{RustEmbed, Embed};

#[derive(RustEmbed)]
#[folder = "../../frontend/dist"] // 指向Vue项目的dist目录（相对 plugins/plugin_tihc_backend/src/handlers/）
pub struct StaticFiles;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
	let path = uri.path().trim_start_matches('/');
	let path = if path.is_empty() { "index.html" } else { path };

	// 打印 embed 的所有文件名和当前请求的 path
	let all_files: Vec<_> = <StaticFiles as Embed>::iter().collect();
	tracing::debug!(target: "static_files", "static_handler: request path = {}, embed files = {:?}", path, all_files);

	if let Some(content) = <StaticFiles as Embed>::get(path) {
		let mime_type = mime_guess::from_path(path).first_or_octet_stream();
		return Response::builder()
			.status(StatusCode::OK)
			.header(
				header::CONTENT_TYPE,
				HeaderValue::from_str(mime_type.as_ref()).unwrap(),
			)
			.body(Body::from(content.data.to_vec()))
			.unwrap();
	}

	// For SPA routing, serve index.html for unknown routes
	if let Some(content) = <StaticFiles as Embed>::get("index.html") {
		Response::builder()
			.status(StatusCode::OK)
			.header(
				header::CONTENT_TYPE,
				HeaderValue::from_str("text/html").unwrap(),
			)
			.body(Body::from(content.data.to_vec()))
			.unwrap()
	} else {
		Response::builder()
			.status(StatusCode::NOT_FOUND)
			.body(Body::from("404 Not Found"))
			.unwrap()
	}
}

pub async fn index_handler() -> impl IntoResponse {
	static_handler("/".parse().unwrap()).await
}
