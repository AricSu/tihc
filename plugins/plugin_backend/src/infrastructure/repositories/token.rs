use crate::domain::auth::token::TokenRepository;

// 实现领域层 TokenRepository trait
#[async_trait::async_trait]
impl TokenRepository for MySqlTokenRepository {
        async fn revoke_all_tokens_for_user(&self, user_id: i64) -> crate::domain::shared::DomainResult<()> {
            sqlx::query(
                r#"UPDATE auth_tokens SET revoked = 1, revoked_at = NOW(), updated_at = NOW() WHERE user_id = ? AND revoked = 0"#
            )
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| crate::domain::shared::DomainError::InternalError { message: format!("Database error: {}", e) })?;
            Ok(())
        }
    async fn store_token(&self, user_id: i64, token: &str, expires_at: DateTime<Utc>) -> crate::domain::shared::DomainResult<()> {
        self.store_token(user_id, token, expires_at).await
    }
    async fn revoke_token(&self, token: &str) -> crate::domain::shared::DomainResult<()> {
        self.revoke_token(token).await
    }

    async fn is_token_active(&self, token: &str, user_id: i64) -> crate::domain::shared::DomainResult<bool> {
        let result = sqlx::query_scalar::<sqlx::mysql::MySql, bool>(
            r#"SELECT NOT revoked AND expires_at > NOW() FROM tihc_tokens WHERE token = ? AND user_id = ?"#
        )
        .bind(token)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::domain::shared::DomainError::InternalError { message: format!("Database error: {}", e) })?;
        Ok(result.unwrap_or(false))
    }
}
use chrono::{DateTime, Utc};
use sqlx::{MySqlPool};
use crate::domain::shared::{DomainError, DomainResult};


pub struct MySqlTokenRepository {
    pool: MySqlPool,
}

impl MySqlTokenRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn store_token(&self, user_id: i64, token: &str, expires_at: DateTime<Utc>) -> DomainResult<()> {
        sqlx::query(
            r#"
            INSERT INTO tihc_tokens (user_id, token, expires_at, revoked, created_at, updated_at)
            VALUES (?, ?, ?, false, NOW(), NOW())
            ON DUPLICATE KEY UPDATE expires_at = VALUES(expires_at), revoked = false, updated_at = NOW()
            "#
        )
        .bind(user_id)
        .bind(token)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { message: format!("Database error: {}", e) })?;
        Ok(())
    }

    pub async fn revoke_token(&self, token: &str) -> DomainResult<()> {
        sqlx::query(
            r#"
            UPDATE tihc_tokens SET revoked = true, updated_at = NOW() WHERE token = ?
            "#
        )
        .bind(token)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { message: format!("Database error: {}", e) })?;
        Ok(())
    }

}

