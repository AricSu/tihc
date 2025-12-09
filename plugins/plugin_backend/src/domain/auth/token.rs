use chrono::{DateTime, Utc};
use async_trait::async_trait;
use crate::domain::shared::DomainResult;

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn store_token(&self, user_id: i64, token: &str, expires_at: DateTime<Utc>) -> DomainResult<()>;
    async fn revoke_token(&self, token: &str) -> DomainResult<()>;
    async fn is_token_active(&self, token: &str, user_id: i64) -> DomainResult<bool>;
}
