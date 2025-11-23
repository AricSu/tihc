use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{MySqlPool, Row};

use crate::domain::auth::AuthTokenStore;
use crate::domain::shared::{DomainError, DomainResult};

pub struct AuthTokenRepository {
    pool: MySqlPool,
}

impl AuthTokenRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    #[inline]
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let digest = hasher.finalize();
        format!("{:x}", digest)
    }

    fn map_db_error(message: impl Into<String>, error: sqlx::Error) -> DomainError {
        DomainError::InternalError {
            message: format!("{}: {}", message.into(), error),
        }
    }

    pub async fn cleanup_expired(&self) -> DomainResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM auth_tokens
            WHERE expires_at < CURRENT_TIMESTAMP
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Self::map_db_error("Failed to cleanup expired tokens", e))?;

        Ok(result.rows_affected())
    }
}

#[async_trait]
impl AuthTokenStore for AuthTokenRepository {
    async fn store_token(
        &self,
        user_id: i64,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> DomainResult<()> {
        let token_hash = Self::hash_token(token);
        sqlx::query(
            r#"
            INSERT INTO auth_tokens (user_id, token_hash, expires_at)
            VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE
                user_id = VALUES(user_id),
                expires_at = VALUES(expires_at),
                revoked = 0,
                revoked_at = NULL
            "#,
        )
        .bind(user_id)
        .bind(&token_hash)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| Self::map_db_error("Failed to store token", e))?;

        Ok(())
    }

    async fn is_token_active(&self, token: &str) -> DomainResult<bool> {
        let token_hash = Self::hash_token(token);
        let row = sqlx::query(
            r#"
            SELECT revoked, expires_at
            FROM auth_tokens
            WHERE token_hash = ?
            LIMIT 1
            "#,
        )
        .bind(&token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Self::map_db_error("Failed to query token", e))?;

        if let Some(row) = row {
            let revoked: i8 = row.get("revoked");
            let expires_at: DateTime<Utc> = row.get("expires_at");
            Ok(revoked == 0 && expires_at > Utc::now())
        } else {
            Ok(false)
        }
    }

    async fn revoke_token(&self, token: &str) -> DomainResult<()> {
        let token_hash = Self::hash_token(token);
        sqlx::query(
            r#"
            UPDATE auth_tokens
            SET revoked = 1, revoked_at = CURRENT_TIMESTAMP
            WHERE token_hash = ?
            "#,
        )
        .bind(&token_hash)
        .execute(&self.pool)
        .await
        .map_err(|e| Self::map_db_error("Failed to revoke token", e))?;

        Ok(())
    }
}
