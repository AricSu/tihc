use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tower::ServiceBuilder;

pub fn api_middleware() -> ServiceBuilder<impl tower::Layer<axum::Router>> {
    ServiceBuilder::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
}
