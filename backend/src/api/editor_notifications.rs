use axum::{
    Router,
    response::sse::{Event, Sse},
    routing::get,
};
use futures::stream;

use axum::response::IntoResponse;

use std::time::Duration;
use tokio_stream::StreamExt;

async fn notifications() -> impl IntoResponse {
    let events = stream::iter(vec![
        Ok::<Event, std::convert::Infallible>(Event::default().data(r#"{"status": "running", "progress": 10, "message": "SQL is being executed..."}"#)),
        Ok::<Event, std::convert::Infallible>(Event::default().data(r#"{"status": "running", "progress": 50, "message": "SQL is halfway done..."}"#)),
        Ok::<Event, std::convert::Infallible>(Event::default().data(r#"{"status": "completed", "progress": 100, "message": "SQL query executed successfully"}"#)),
    ]).throttle(Duration::from_millis(800));
    Sse::new(events)
}

pub fn routes() -> Router {
    Router::new().route("/api/notifications", get(notifications))
}
