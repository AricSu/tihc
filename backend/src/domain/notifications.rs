use serde::{Deserialize, Serialize};

/// Represents the status of a task or operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
    Pending,
}

/// Represents a notification event in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub status: TaskStatus,
    pub progress: u8, // 0-100
    pub message: String,
}

impl NotificationEvent {
    /// Creates a new notification event
    pub fn new(status: TaskStatus, progress: u8, message: String) -> Self {
        Self {
            status,
            progress: progress.min(100), // Ensure progress doesn't exceed 100
            message,
        }
    }

    /// Creates a running status notification
    pub fn running(progress: u8, message: String) -> Self {
        Self::new(TaskStatus::Running, progress, message)
    }

    /// Creates a completed status notification
    pub fn completed(message: String) -> Self {
        Self::new(TaskStatus::Completed, 100, message)
    }

    /// Creates a failed status notification
    pub fn failed(message: String) -> Self {
        Self::new(TaskStatus::Failed, 0, message)
    }

    /// Creates a pending status notification
    pub fn pending(message: String) -> Self {
        Self::new(TaskStatus::Pending, 0, message)
    }

    /// Converts the event to JSON string for SSE transmission
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_event_creation() {
        let event = NotificationEvent::running(50, "Processing...".to_string());
        
        assert_eq!(event.status, TaskStatus::Running);
        assert_eq!(event.progress, 50);
        assert_eq!(event.message, "Processing...");
    }

    #[test]
    fn test_progress_clamping() {
        let event = NotificationEvent::new(TaskStatus::Running, 150, "Test".to_string());
        assert_eq!(event.progress, 100);
    }

    #[test]
    fn test_completed_event() {
        let event = NotificationEvent::completed("Done!".to_string());
        
        assert_eq!(event.status, TaskStatus::Completed);
        assert_eq!(event.progress, 100);
        assert_eq!(event.message, "Done!");
    }

    #[test]
    fn test_json_serialization() {
        let event = NotificationEvent::running(25, "Working...".to_string());
        let json = event.to_json().unwrap();
        
        assert!(json.contains("\"status\":\"running\""));
        assert!(json.contains("\"progress\":25"));
        assert!(json.contains("\"message\":\"Working...\""));
    }
}
