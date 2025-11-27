use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub title: String,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChatSessionRequest {
    #[serde(alias = "userId")]
    pub user_id: i64,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChatSessionsRequest {
    #[serde(alias = "userId")]
    pub user_id: i64,
    pub limit: Option<i32>,
}
