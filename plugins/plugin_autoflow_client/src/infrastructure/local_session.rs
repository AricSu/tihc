use chrono::{Duration as ChronoDuration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::session::{SessionRepository, SessionState};

/// 内存会话管理器 - 基础设施层实现
#[derive(Debug, Clone)]
pub struct InMemorySessionRepository {
    inner: Arc<RwLock<HashMap<String, SessionState>>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn create_session(&self) -> SessionState {
        let state = SessionState::new(uuid::Uuid::new_v4().to_string());
        let clone = state.clone();
        let mut guard = self.inner.write().await;
        guard.insert(clone.session_id.clone(), clone.clone());
        state
    }

    async fn insert_existing(&self, state: SessionState) {
        let mut guard = self.inner.write().await;
        guard.insert(state.session_id.clone(), state);
    }

    async fn get(&self, session_id: &str) -> Option<SessionState> {
        let mut guard = self.inner.write().await;
        guard.get_mut(session_id).map(|state| {
            state.touch();
            state.clone()
        })
    }

    async fn update_remote(
        &self,
        session_id: &str,
        chat_id: Option<String>,
    ) -> Option<SessionState> {
        let mut guard = self.inner.write().await;
        if let Some(state) = guard.get_mut(session_id) {
            state.set_remote(chat_id);
            return Some(state.clone());
        }
        None
    }

    async fn close(&self, session_id: &str) -> bool {
        let mut guard = self.inner.write().await;
        guard.remove(session_id).is_some()
    }

    async fn list(&self) -> Vec<SessionState> {
        let guard = self.inner.read().await;
        guard.values().cloned().collect()
    }

    async fn cleanup_expired(&self, ttl_minutes: i64) -> usize {
        let cutoff = Utc::now() - ChronoDuration::minutes(ttl_minutes);
        let mut guard = self.inner.write().await;
        let expired: Vec<String> = guard
            .iter()
            .filter(|(_, state)| state.updated_at < cutoff)
            .map(|(id, _)| id.clone())
            .collect();

        for id in &expired {
            guard.remove(id);
        }

        expired.len()
    }

    async fn find_by_chat_id(&self, chat_id: &str) -> Option<SessionState> {
        let mut guard = self.inner.write().await;
        guard
            .iter_mut()
            .find(|(_, state)| state.matches_conversation(chat_id))
            .map(|(_, state)| {
                state.touch();
                state.clone()
            })
    }

    async fn remove_by_chat_id(&self, chat_id: &str) -> usize {
        let mut guard = self.inner.write().await;
        let ids: Vec<String> = guard
            .iter()
            .filter(|(_, state)| state.matches_conversation(chat_id))
            .map(|(id, _)| id.clone())
            .collect();

        let count = ids.len();
        for id in ids {
            guard.remove(&id);
        }

        count
    }
}