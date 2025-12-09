use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::domain::shared::DomainResult;
use crate::domain::auth::token::TokenRepository;

pub struct TokenService<R: TokenRepository + ?Sized> {
    pub repository: Arc<R>,
}

impl<R: TokenRepository + ?Sized> TokenService<R> {
    /// 撤销某用户所有未撤销 token
    pub async fn revoke_all_tokens_for_user(&self, user_id: i64) -> DomainResult<()> {
        self.repository.revoke_all_tokens_for_user(user_id).await
    }

        /// 检查 token 是否有效（未吊销且未过期）
        pub async fn is_token_active(&self, token: &str, user_id: i64) -> DomainResult<bool> {
            self.repository.is_token_active(token, user_id).await
        }
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn store_token(&self, user_id: i64, token: &str, expires_at: DateTime<Utc>) -> DomainResult<()> {
        self.repository.store_token(user_id, token, expires_at).await
    }

    pub async fn revoke_token(&self, token: &str) -> DomainResult<()> {
        self.repository.revoke_token(token).await
    }

}
