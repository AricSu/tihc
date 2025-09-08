// Health Check HTTP Controllers

use axum::{routing::get, Router};

pub struct HealthController;

impl HealthController {
    /// 创建健康检查路由
    pub fn routes() -> Router<()> {
        Router::new().route("/healthz", get(healthz_handler))
    }
}

/// 健康检查处理函数
async fn healthz_handler() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_healthz_endpoint() {
        let app = HealthController::routes();

        let request = Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"ok");
    }
}
