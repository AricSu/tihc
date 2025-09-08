use std::time::Duration;
use futures::stream::{self, Stream};
use tokio_stream::StreamExt;
use crate::domain::notifications::NotificationEvent;

/// Application service for managing notifications and SSE streams
pub struct NotificationsApplicationService;

impl NotificationsApplicationService {
    /// Creates a new notifications application service
    pub fn new() -> Self {
        Self
    }

    /// Creates a sample notification stream for demonstration
    /// In a real application, this would connect to a message bus or event store
    pub fn create_sample_notification_stream(&self) -> impl Stream<Item = Result<NotificationEvent, std::convert::Infallible>> {
        let events = vec![
            NotificationEvent::running(10, "SQL is being executed...".to_string()),
            NotificationEvent::running(50, "SQL is halfway done...".to_string()),
            NotificationEvent::completed("SQL query executed successfully".to_string()),
        ];

        stream::iter(events)
            .map(|event| Ok(event))
            .throttle(Duration::from_millis(800))
    }

    /// Creates a notification stream for a specific task
    /// This is where you would integrate with your actual notification system
    pub fn create_task_notification_stream(&self, _task_id: &str) -> impl Stream<Item = Result<NotificationEvent, std::convert::Infallible>> {
        // For now, return the sample stream
        // In the future, this would fetch real task status from a data store
        self.create_sample_notification_stream()
    }
}

impl Default for NotificationsApplicationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;
    use futures::pin_mut;

    #[tokio::test]
    async fn test_sample_notification_stream() {
        let service = NotificationsApplicationService::new();
        let stream = service.create_sample_notification_stream();
        pin_mut!(stream);

        // Collect the first event
        let first_event = stream.next().await.unwrap().unwrap();
        assert_eq!(first_event.progress, 10);
        assert!(first_event.message.contains("SQL is being executed"));
    }

    #[tokio::test]
    async fn test_task_notification_stream() {
        let service = NotificationsApplicationService::new();
        let stream = service.create_task_notification_stream("test-task-123");
        pin_mut!(stream);

        // Verify we get at least one event
        let event = stream.next().await;
        assert!(event.is_some());
    }
}
