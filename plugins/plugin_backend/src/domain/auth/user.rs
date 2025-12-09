use crate::domain::shared::{DomainError, DomainResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 根据用户名查找用户
    async fn find_by_username(&self, username: &str) -> DomainResult<Option<User>>;

    /// 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>>;

    /// 根据 ID 查找用户
    async fn find_by_id(&self, user_id: i64) -> DomainResult<Option<User>>;

    /// 根据第三方提供商查找用户
    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> DomainResult<Option<User>>;

    /// 根据 GitHub name 查找用户
    async fn find_by_github_name(&self, github_name: &str) -> DomainResult<Option<User>>;

    /// 分页查询用户
    async fn find_users_paginated(
        &self,
        page: i32,
        page_size: i32,
        keyword: Option<String>,
        status: Option<String>,
    ) -> DomainResult<(Vec<User>, i64)>;

    /// 保存用户
    async fn save(&self, user: &User) -> DomainResult<i64>;

    /// 更新用户
    async fn update(&self, user: &User) -> DomainResult<()>;
}

/// 用户提供商仓储接口
#[async_trait]
pub trait UserProviderRepository: Send + Sync {
    /// 保存用户提供商关联
    async fn save(&self, provider: &UserProvider) -> DomainResult<()>;
}

/// 用户实体：纯业务对象，不包含基础设施关注点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>, // 新创建的用户 ID 为 None
    pub user_id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub github_name: Option<String>, // GitHub 显示名称，用于匹配
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active = 1,
    Inactive = 0,
    Banned = -1,
}

impl From<i8> for UserStatus {
    fn from(status: i8) -> Self {
        match status {
            1 => UserStatus::Active,
            -1 => UserStatus::Banned,
            _ => UserStatus::Inactive,
        }
    }
}

impl From<UserStatus> for i8 {
    fn from(status: UserStatus) -> Self {
        status as i8
    }
}

impl User {
    /// 创建新用户
    pub fn new(
        username: String,
        password_hash: String,
        email: String,
        nick_name: Option<String>,
        avatar: Option<String>,
    ) -> DomainResult<Self> {
        // 业务规则验证
        if username.is_empty() || username.len() < 3 {
            return Err(DomainError::ValidationError {
                message: "用户名长度至少 3 个字符".to_string(),
            });
        }

        if email.is_empty() || !email.contains('@') {
            return Err(DomainError::ValidationError {
                message: "邮箱格式无效".to_string(),
            });
        }

        if password_hash.is_empty() {
            return Err(DomainError::ValidationError {
                message: "密码哈希不能为空".to_string(),
            });
        }

        let now = Utc::now();
        Ok(Self {
            id: None,
            user_id: 0,
            username,
            password_hash,
            email,
            nick_name,
            avatar,
            github_name: None, // 初始创建时为空，通过 GitHub 登录时会设置
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 更新用户信息
    pub fn update_profile(&mut self, nick_name: Option<String>, avatar: Option<String>) {
        self.nick_name = nick_name;
        self.avatar = avatar;
        self.updated_at = Utc::now();
    }

    /// 检查用户是否可用
    pub fn is_available(&self) -> bool {
        self.status == "active"
    }
}

/// 用户信息值对象：用于对外展示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub status: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        let id = user.id.unwrap_or(0);
        Self {
            id,
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            nick_name: user.nick_name,
            avatar: user.avatar,
            status: user.status,
        }
    }
}

/// 用户第三方提供商关联实体：纯业务对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProvider {
    pub id: Option<i64>,
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

impl UserProvider {
    /// 创建新的用户提供商关联
    pub fn new(
        user_id: i64,
        provider: String,
        provider_user_id: String,
        provider_email: Option<String>,
        provider_raw: Option<serde_json::Value>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            user_id,
            provider,
            provider_user_id,
            provider_email,
            provider_raw,
            refresh_token_encrypted: None,
            scope: None,
            token_expires_at: None,
            last_synced_at: Some(now),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}
