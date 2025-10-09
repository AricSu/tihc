use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionRequest {
    pub time_range: [i64; 2], // [start_timestamp, end_timestamp]
    pub timezone: String,
    pub clinic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionResponse {
    pub success: bool,
    pub task_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionTask {
    pub task_id: String,
    pub time_range: [i64; 2],
    pub timezone: String,
    pub clinic_url: String,
    pub status: TaskStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,
    Running,
    Completed,
    Failed,
}

impl InspectionResponse {
    pub fn success(task_id: String) -> Self {
        Self {
            success: true,
            task_id: Some(task_id),
            message: "巡检任务创建成功".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            task_id: None,
            message,
        }
    }
}

impl InspectionTask {
    pub fn new(request: InspectionRequest) -> Self {
        let task_id = format!("inspection_{}", chrono::Utc::now().timestamp_millis());
        let now = chrono::Utc::now().timestamp();

        Self {
            task_id,
            time_range: request.time_range,
            timezone: request.timezone,
            clinic_url: request.clinic_url,
            status: TaskStatus::Created,
            created_at: now,
            updated_at: now,
        }
    }
}
