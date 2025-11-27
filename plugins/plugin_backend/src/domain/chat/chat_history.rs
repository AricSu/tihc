use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory {
    pub id: i64,
    #[serde(rename = "sessionId")]
    pub session_id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "user")]
    pub user_message: String,
    #[serde(rename = "assistant")]
    pub assistant_message: String,
    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistoryRequest {
    #[serde(alias = "userId")]
    pub user_id: i64,
    #[serde(alias = "sessionId")]
    pub session_id: Option<i64>,
    pub limit: Option<i32>,
}
