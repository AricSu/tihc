use chrono::{DateTime, Utc};
use std::collections::HashSet;

/// 会话状态 - 领域实体
#[derive(Debug, Clone)]
pub struct SessionState {
    pub session_id: String,
    pub chat_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    conversation_aliases: HashSet<String>,
}

impl SessionState {
    pub fn new(session_id: String) -> Self {
        let now = Utc::now();
        Self {
            session_id,
            chat_id: None,
            created_at: now,
            updated_at: now,
            conversation_aliases: HashSet::new(),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn set_remote(
        &mut self,
        chat_id: Option<String>,
    ) {
        if chat_id.is_some() {
            if let Some(id) = chat_id {
                self.conversation_aliases.insert(id.clone());
                self.chat_id = Some(id);
            }
        }

        self.touch();
    }

    pub fn matches_conversation(&self, chat_id: &str) -> bool {
        self.chat_id
            .as_deref()
            .map(|id| id == chat_id)
            .unwrap_or(false)
            || self.conversation_aliases.contains(chat_id)
    }
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create_session(&self) -> SessionState;
    async fn insert_existing(&self, state: SessionState);
    async fn get(&self, session_id: &str) -> Option<SessionState>;
    async fn update_remote(
        &self,
        session_id: &str,
        chat_id: Option<String>
    ) -> Option<SessionState>;
    async fn close(&self, session_id: &str) -> bool;
    async fn list(&self) -> Vec<SessionState>;
    async fn cleanup_expired(&self, ttl_minutes: i64) -> usize;
    async fn find_by_chat_id(&self, chat_id: &str) -> Option<SessionState>;
    async fn remove_by_chat_id(&self, chat_id: &str) -> usize;
}