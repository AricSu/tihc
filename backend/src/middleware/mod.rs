//! Global middleware constructors for axum/tower.
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// Returns a CORS middleware layer allowing any origin, method, and header.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Returns a HTTP trace middleware layer for logging requests/responses.
pub fn trace_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
}
