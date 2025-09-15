use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Sse},
    routing::get,
    Router,
};
use futures::stream::StreamExt;
use std::convert::Infallible;
use std::sync::Arc;

use crate::application::notifications::NotificationsApplicationService;

/// Controller for handling notification-related HTTP endpoints
pub struct NotificationsController;

impl NotificationsController {
    /// Creates the notifications router with all endpoints
    pub fn routes() -> Router<NotificationsAppState> {
        Router::new().route("/notifications", get(notifications_handler))
    }
}

/// 通知应用状态
#[derive(Clone)]
pub struct NotificationsAppState {
    pub notifications_service: Arc<NotificationsApplicationService>,
}

impl NotificationsAppState {
    pub fn new(notifications_service: Arc<NotificationsApplicationService>) -> Self {
        Self {
            notifications_service,
        }
    }
}

/// Handler for SSE notifications endpoint
async fn notifications_handler(State(state): State<NotificationsAppState>) -> impl IntoResponse {
    let stream = state
        .notifications_service
        .create_sample_notification_stream();

    // Convert notification events to SSE events
    let sse_stream =
        stream.map(|result| match result {
            Ok(notification) => match notification.to_json() {
                Ok(json) => Ok::<Event, Infallible>(Event::default().data(json)),
                Err(_) => Ok(Event::default()
                    .data(r#"{"status":"failed","progress":0,"message":"Serialization error"}"#)),
            },
            Err(_) => Ok(Event::default()
                .data(r#"{"status":"failed","progress":0,"message":"Stream error"}"#)),
        });

    Sse::new(sse_stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    fn create_test_app_state() -> NotificationsAppState {
        let notifications_service = Arc::new(NotificationsApplicationService::new());
        NotificationsAppState::new(notifications_service)
    }

    #[tokio::test]
    async fn test_notifications_endpoint() {
        let app_state = create_test_app_state();
        let app = NotificationsController::routes().with_state(app_state);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/notifications")
                    .header("Accept", "text/event-stream")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/event-stream"
        );
    }
}
