use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

use crate::domain::auth::{User, UserProvider, UserProviderRepository, UserRepository};
use crate::domain::shared::{DomainError, DomainResult};

/// 数据库用户 DTO：包含持久化关注点
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct UserDto {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub github_name: Option<String>,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            id: Some(dto.id),
            user_id: dto.user_id,
            username: dto.username,
            password_hash: dto.password_hash,
            email: dto.email,
            nick_name: dto.nick_name,
            avatar: dto.avatar,
            status: match dto.status {
                1 => "active".to_string(),
                -1 => "banned".to_string(),
                _ => "inactive".to_string(),
            },
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            github_name: dto.github_name,
        }
    }
}

/// 数据库用户提供商 DTO
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
struct UserProviderDto {
    pub id: i64,
    pub user_id: i64,
    pub provider: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    pub provider_raw: Option<serde_json::Value>,
    pub refresh_token_encrypted: Option<String>,
    pub scope: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserProviderDto> for UserProvider {
    fn from(dto: UserProviderDto) -> Self {
        Self {
            id: Some(dto.id),
            user_id: dto.user_id,
            provider: dto.provider,
            provider_user_id: dto.provider_user_id,
            provider_email: dto.provider_email,
            provider_raw: dto.provider_raw,
            refresh_token_encrypted: dto.refresh_token_encrypted,
            scope: dto.scope,
            token_expires_at: dto.token_expires_at,
            last_synced_at: dto.last_synced_at,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<UserProvider> for UserProviderDto {
    fn from(provider: UserProvider) -> Self {
        Self {
            id: provider.id.unwrap_or(0),
            user_id: provider.user_id,
            provider: provider.provider,
            provider_user_id: provider.provider_user_id,
            provider_email: provider.provider_email,
            provider_raw: provider.provider_raw,
            refresh_token_encrypted: provider.refresh_token_encrypted,
            scope: provider.scope,
            token_expires_at: provider.token_expires_at,
            last_synced_at: provider.last_synced_at,
            created_at: provider.created_at,
            updated_at: provider.updated_at,
        }
    }
}

/// MySQL 用户仓储实现
pub struct MySqlUserRepository {
    pool: MySqlPool,
}

impl MySqlUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn find_by_username(&self, username: &str) -> DomainResult<Option<User>> {
        let dto = sqlx::query_as::<_, UserDto>(
            "SELECT id, user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at FROM tihc_users WHERE username = ? AND status = 1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        let dto = sqlx::query_as::<_, UserDto>(
            "SELECT id, user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at FROM tihc_users WHERE email = ? AND status = 1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(User::from))
    }

    async fn find_by_id(&self, user_id: i64) -> DomainResult<Option<User>> {
        let dto = sqlx::query_as::<_, UserDto>(
            "SELECT id, user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at FROM tihc_users WHERE id = ? AND status = 1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(User::from))
    }

    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> DomainResult<Option<User>> {
        let dto = sqlx::query_as::<_, UserDto>(
            r#"
            SELECT u.id, u.user_id, u.username, u.password_hash, u.email, u.nick_name, u.avatar, u.github_name, u.status, u.created_at, u.updated_at 
            FROM tihc_users u 
            INNER JOIN tihc_user_providers p ON u.id = p.user_id 
            WHERE p.provider = ? AND p.provider_user_id = ? AND u.status = 1
            "#
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(User::from))
    }

    async fn save(&self, user: &User) -> DomainResult<i64> {
        let provided_user_id = if user.user_id == 0 {
            None
        } else {
            Some(user.user_id)
        };

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::InternalError {
                message: format!("Database error: {}", e),
            })?;

        let result = sqlx::query(
            "INSERT INTO tihc_users (user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(provided_user_id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.email)
        .bind(&user.nick_name)
        .bind(&user.avatar)
        .bind(&user.github_name)
        .bind(match user.status.as_str() {
            "active" => 1i8,
            "banned" => -1i8,
            _ => 0i8,
        })
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        let new_id = result.last_insert_id() as i64;

        if provided_user_id.is_none() {
            sqlx::query("UPDATE tihc_users SET user_id = ? WHERE id = ? AND user_id IS NULL")
                .bind(new_id)
                .bind(new_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| DomainError::InternalError {
                    message: format!("Database error: {}", e),
                })?;
        }

        tx.commit().await.map_err(|e| DomainError::InternalError {
            message: format!("Database error: {}", e),
        })?;

        Ok(new_id)
    }

    async fn update(&self, user: &User) -> DomainResult<()> {
        let user_id = user.id.ok_or_else(|| DomainError::ValidationError {
            message: "用户 ID 不能为空".to_string(),
        })?;

        sqlx::query(
            "UPDATE tihc_users SET user_id = ?, username = ?, password_hash = ?, email = ?, nick_name = ?, avatar = ?, github_name = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(user.user_id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.email)
        .bind(&user.nick_name)
        .bind(&user.avatar)
        .bind(&user.github_name)
        .bind(match user.status.as_str() {
            "active" => 1i8,
            "banned" => -1i8,
            _ => 0i8,
        })
        .bind(user.updated_at)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(())
    }

    async fn find_by_github_name(&self, github_name: &str) -> DomainResult<Option<User>> {
        let dto = sqlx::query_as::<_, UserDto>(
            "SELECT id, user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at FROM tihc_users WHERE github_name = ? AND status = 1"
        )
        .bind(github_name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(User::from))
    }

    async fn delete(&self, user_id: i64) -> DomainResult<()> {
        sqlx::query(
            "UPDATE tihc_users SET status = 0, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError {
            message: format!("Database error: {}", e),
        })?;

        Ok(())
    }

    async fn find_users_paginated(
        &self,
        page: i32,
        page_size: i32,
        keyword: Option<String>,
        status: Option<String>,
    ) -> DomainResult<(Vec<User>, i64)> {
        let offset = (page - 1) * page_size;

        // 构建基础查询
        let mut query = "SELECT id, user_id, username, password_hash, email, nick_name, avatar, github_name, status, created_at, updated_at FROM tihc_users WHERE 1=1".to_string();
        let mut count_query = "SELECT COUNT(*) FROM tihc_users WHERE 1=1".to_string();

        // 添加条件
        if let Some(ref status_filter) = status {
            // map status string to numeric DB value
            let status_num: i8 = match status_filter.as_str() {
                "active" => 1,
                "banned" => -1,
                _ => 0,
            };
            query += &format!(" AND status = {}", status_num);
            count_query += &format!(" AND status = {}", status_num);
        }

        if keyword.is_some() {
            query += " AND (username LIKE CONCAT('%', ?, '%') OR email LIKE CONCAT('%', ?, '%') OR nick_name LIKE CONCAT('%', ?, '%'))";
            count_query += " AND (username LIKE CONCAT('%', ?, '%') OR email LIKE CONCAT('%', ?, '%') OR nick_name LIKE CONCAT('%', ?, '%'))";
        }

        // 添加分页
        query += " ORDER BY created_at DESC LIMIT ? OFFSET ?";

        // 执行查询
        let mut users_query = sqlx::query_as::<_, UserDto>(&query);
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_query);

        if let Some(ref keyword) = keyword {
            users_query = users_query.bind(keyword).bind(keyword).bind(keyword);
            count_query = count_query.bind(keyword).bind(keyword).bind(keyword);
        }

        users_query = users_query.bind(page_size).bind(offset);

        // 并行执行查询和计数
        let (users_dto, total) = tokio::try_join!(
            users_query.fetch_all(&self.pool),
            count_query.fetch_one(&self.pool)
        )
        .map_err(|e| DomainError::InternalError {
            message: format!("Database error: {}", e),
        })?;

        let users = users_dto.into_iter().map(User::from).collect();

        Ok((users, total))
    }
}

/// MySQL 用户提供商仓储实现
pub struct MySqlUserProviderRepository {
    pool: MySqlPool,
}

impl MySqlUserProviderRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProviderRepository for MySqlUserProviderRepository {
    async fn save(&self, provider: &UserProvider) -> DomainResult<()> {
        sqlx::query(
            r#"
            INSERT INTO tihc_user_providers (user_id, provider, provider_user_id, provider_email, provider_raw, refresh_token_encrypted, scope, token_expires_at, last_synced_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE 
                provider_email = VALUES(provider_email),
                provider_raw = VALUES(provider_raw),
                refresh_token_encrypted = VALUES(refresh_token_encrypted),
                scope = VALUES(scope),
                token_expires_at = VALUES(token_expires_at),
                last_synced_at = VALUES(last_synced_at),
                updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(provider.user_id)
        .bind(&provider.provider)
        .bind(&provider.provider_user_id)
        .bind(&provider.provider_email)
        .bind(&provider.provider_raw)
        .bind(&provider.refresh_token_encrypted)
        .bind(&provider.scope)
        .bind(provider.token_expires_at)
        .bind(provider.last_synced_at)
        .bind(provider.created_at)
        .bind(provider.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(())
    }

    async fn find_by_user_and_provider(
        &self,
        user_id: i64,
        provider: &str,
    ) -> DomainResult<Option<UserProvider>> {
        let dto = sqlx::query_as::<_, UserProviderDto>(
            "SELECT id, user_id, provider, provider_user_id, provider_email, provider_raw, refresh_token_encrypted, scope, token_expires_at, last_synced_at, created_at, updated_at FROM tihc_user_providers WHERE user_id = ? AND provider = ?"
        )
        .bind(user_id)
        .bind(provider)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(dto.map(UserProvider::from))
    }

    async fn update(&self, provider: &UserProvider) -> DomainResult<()> {
        let provider_id = provider.id.ok_or_else(|| DomainError::ValidationError {
            message: "用户提供商 ID 不能为空".to_string(),
        })?;

        sqlx::query(
            "UPDATE tihc_user_providers SET provider_email = ?, provider_raw = ?, refresh_token_encrypted = ?, scope = ?, token_expires_at = ?, last_synced_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&provider.provider_email)
        .bind(&provider.provider_raw)
        .bind(&provider.refresh_token_encrypted)
        .bind(&provider.scope)
        .bind(provider.token_expires_at)
        .bind(provider.last_synced_at)
        .bind(provider.updated_at)
        .bind(provider_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError { 
            message: format!("Database error: {}", e) 
        })?;

        Ok(())
    }
}
