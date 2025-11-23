use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::shared::DomainResult;

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub id: Option<i64>,
    pub user_id: i64,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait AuthTokenStore: Send + Sync {
    async fn store_token(
        &self,
        user_id: i64,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> DomainResult<()>;

    async fn is_token_active(&self, token: &str) -> DomainResult<bool>;

    async fn revoke_token(&self, token: &str) -> DomainResult<()>;
}
