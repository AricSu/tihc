// HTTP Middleware

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};

/// 创建CORS中间件
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}

/// 创建跟踪中间件
pub fn trace_layer(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true))
}
