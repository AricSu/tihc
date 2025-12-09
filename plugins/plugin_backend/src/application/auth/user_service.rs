use std::sync::Arc;

use crate::domain::auth::{
    GitHubUser, GoogleUser, User, UserProvider, UserProviderRepository, UserRepository,
};
use crate::domain::shared::{
    DomainError, DomainResult,
    services::{PasswordService, UuidService},
};

/// 应用层用户服务：编排业务用例，不直接操作数据库
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    provider_repository: Arc<dyn UserProviderRepository>,
    password_service: Arc<dyn PasswordService>,
    uuid_service: Arc<dyn UuidService>,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        provider_repository: Arc<dyn UserProviderRepository>,
        password_service: Arc<dyn PasswordService>,
        uuid_service: Arc<dyn UuidService>,
    ) -> Self {
        Self {
            user_repository,
            provider_repository,
            password_service,
            uuid_service,
        }
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> DomainResult<Option<User>> {
        self.user_repository.find_by_username(username).await
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        self.user_repository.find_by_email(email).await
    }

    /// 根据 ID 查找用户
    pub async fn find_by_id(&self, user_id: i64) -> DomainResult<Option<User>> {
        self.user_repository.find_by_id(user_id).await
    }

    /// 验证密码
    pub fn verify_password(&self, password: &str, password_hash: &str) -> DomainResult<bool> {
        self.password_service
            .verify_password(password, password_hash)
    }

    /// 哈希密码
    pub fn hash_password(&self, password: &str) -> DomainResult<String> {
        self.password_service.hash_password(password)
    }

    /// 创建新用户
    pub async fn create_user(
        &self,
        username: String,
        password: String,
        email: String,
        nick_name: Option<String>,
    ) -> DomainResult<i64> {
        // 检查用户名是否已存在
        if let Some(_) = self.find_by_username(&username).await? {
            return Err(DomainError::ValidationError {
                message: "用户名已存在".to_string(),
            });
        }

        // 检查邮箱是否已存在
        if let Some(_) = self.find_by_email(&email).await? {
            return Err(DomainError::ValidationError {
                message: "邮箱已存在".to_string(),
            });
        }

        // 哈希密码
        let password_hash = self.hash_password(&password)?;

        // 创建用户实体
        let user = User::new(username, password_hash, email, nick_name, None)?;

        // 保存用户
        self.user_repository.save(&user).await
    }

    /// 根据第三方提供商查找用户
    pub async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> DomainResult<Option<User>> {
        self.user_repository
            .find_by_provider(provider, provider_user_id)
            .await
    }

    /// 创建或更新 GitHub 用户
    ///
    /// 智能用户统一策略：
    /// 1. 优先通过 GitHub Provider ID 查找已关联的用户
    /// 2. 通过邮箱匹配现有用户（支持账户合并）
    /// 3. 智能用户名匹配（忽略大小写、常见变体）
    /// 4. 手动匹配提示（记录日志供管理员处理）
    /// 5. 创建新用户
    pub async fn create_or_update_github_user(
        &self,
        github_user: &GitHubUser,
    ) -> DomainResult<i64> {
        tracing::info!(
            "Processing GitHub user: login={}, id={}, email={:?}, name={:?}",
            github_user.login,
            github_user.id,
            github_user.email,
            github_user.name
        );

        // 步骤 1: 检查是否已通过 GitHub Provider ID 关联
        if let Some(user) = self
            .find_by_provider("github", &github_user.id.to_string())
            .await?
        {
            if let Some(user_id) = user.id {
                tracing::info!("Found existing GitHub link for user_id: {}", user_id);
                // 更新用户的 GitHub 信息（可能昵称、头像有变化）
                self.sync_github_user_info(user_id, github_user).await?;
                return Ok(user_id);
            }
        }

        // 步骤 2: 通过邮箱匹配现有用户（账户合并场景）
        if let Some(email) = &github_user.email {
            if let Some(user) = self.find_by_email(email).await? {
                if let Some(user_id) = user.id {
                    tracing::info!(
                        "Found email match for GitHub user {} -> user_id: {}",
                        github_user.login,
                        user_id
                    );
                    // 关联 GitHub 账户到已存在用户
                    self.link_github_provider(user_id, github_user).await?;
                    // 同步用户信息
                    self.sync_github_user_info(user_id, github_user).await?;
                    return Ok(user_id);
                }
            }
        } else {
            tracing::warn!(
                "GitHub user {} has no public email, cannot match by email",
                github_user.login
            );
        }

        // 步骤 3: 通过 GitHub name 匹配现有用户
        if let Some(github_name) = &github_user.name {
            // 首先尝试通过 github_name 字段匹配
            if let Some(user) = self
                .user_repository
                .find_by_github_name(github_name)
                .await?
            {
                if let Some(user_id) = user.id {
                    tracing::info!(
                        "Found GitHub name field match for GitHub user {} (name: {}) -> user_id: {}",
                        github_user.login,
                        github_name,
                        user_id
                    );
                    // 关联 GitHub 账户到已存在用户
                    self.link_github_provider(user_id, github_user).await?;
                    // 同步用户信息
                    self.sync_github_user_info(user_id, github_user).await?;
                    return Ok(user_id);
                }
            }

            // 其次尝试通过用户名匹配 GitHub 显示名称
            if let Some(user) = self.find_by_username(github_name).await? {
                if let Some(user_id) = user.id {
                    tracing::info!(
                        "Found username match for GitHub user {} (name: {}) -> user_id: {}",
                        github_user.login,
                        github_name,
                        user_id
                    );
                    // 关联 GitHub 账户到已存在用户
                    self.link_github_provider(user_id, github_user).await?;
                    // 同步用户信息
                    self.sync_github_user_info(user_id, github_user).await?;
                    return Ok(user_id);
                }
            }
        }

        // 步骤 4: 创建新用户（确保用户名唯一性）
        tracing::warn!(
            "No automatic match found for GitHub user {}. Creating new user. Consider manual linking if this is an existing user.",
            github_user.login
        );
        let base_username = &github_user.login;
        let username = self.generate_unique_username(base_username).await?;

        let fallback_email = format!("{}@github.local", github_user.login);
        let email = github_user.email.as_deref().unwrap_or(&fallback_email);
        let nick_name = github_user.name.clone();

        // OAuth 用户使用随机密码（他们不会使用密码登录）
        let dummy_password = self.uuid_service.generate();

        let user_id = self
            .create_user(username, dummy_password, email.to_string(), nick_name)
            .await?;

        // 设置头像
        if let Some(avatar_url) = &github_user.avatar_url {
            if let Some(mut user) = self.find_by_id(user_id).await? {
                user.update_profile(user.nick_name.clone(), Some(avatar_url.clone()));
                self.user_repository.update(&user).await?;
            }
        }

        // 关联 GitHub 提供商
        self.link_github_provider(user_id, github_user).await?;

        Ok(user_id)
    }

    /// 生成唯一用户名
    async fn generate_unique_username(&self, base_username: &str) -> DomainResult<String> {
        // 优先尝试原始用户名
        if self.find_by_username(base_username).await?.is_none() {
            return Ok(base_username.to_string());
        }

        // 尝试 github_ 前缀
        let github_username = format!("github_{}", base_username);
        if self.find_by_username(&github_username).await?.is_none() {
            return Ok(github_username);
        }

        // 添加数字后缀确保唯一性
        for i in 1..=999 {
            let candidate = format!("github_{}_{}", base_username, i);
            if self.find_by_username(&candidate).await?.is_none() {
                return Ok(candidate);
            }
        }

        Err(DomainError::InternalError {
            message: "无法生成唯一用户名".to_string(),
        })
    }

    /// 同步 GitHub 用户信息到本地用户
    async fn sync_github_user_info(
        &self,
        user_id: i64,
        github_user: &GitHubUser,
    ) -> DomainResult<()> {
        if let Some(mut user) = self.find_by_id(user_id).await? {
            let mut updated = false;

            // 更新 GitHub name（总是同步最新的）
            if user.github_name != github_user.name {
                user.github_name = github_user.name.clone();
                updated = true;
            }

            // 更新昵称：优先保持现有昵称，除非现有昵称为空
            if user.nick_name.is_none() {
                if let Some(github_name) = &github_user.name {
                    user.nick_name = Some(github_name.clone());
                    updated = true;
                }
            }

            // 更新头像（如果 GitHub 有更新）
            if let Some(avatar_url) = &github_user.avatar_url {
                if user.avatar.as_deref() != Some(avatar_url) {
                    user.avatar = Some(avatar_url.clone());
                    updated = true;
                }
            }

            // 如果有更新则保存
            if updated {
                self.user_repository.update(&user).await?;
            }
        }
        Ok(())
    }

    /// 关联 GitHub 提供商
    async fn link_github_provider(
        &self,
        user_id: i64,
        github_user: &GitHubUser,
    ) -> DomainResult<()> {
        let provider_raw =
            serde_json::to_value(github_user).map_err(|e| DomainError::InternalError {
                message: format!("JSON serialization failed: {}", e),
            })?;

        let provider = UserProvider::new(
            user_id,
            "github".to_string(),
            github_user.id.to_string(),
            github_user.email.clone(),
            Some(provider_raw),
        );

        self.provider_repository.save(&provider).await
    }

    /// 创建或更新 Google 用户
    pub async fn create_or_update_google_user(
        &self,
        google_user: &GoogleUser,
    ) -> DomainResult<i64> {
        tracing::info!(
            "Processing Google user: id={}, email={}, name={:?}",
            google_user.id,
            google_user.email,
            google_user.name
        );

        // 步骤 1: 检查是否已通过 Google Provider ID 关联
        if let Some(user) = self.find_by_provider("google", &google_user.id).await? {
            if let Some(user_id) = user.id {
                tracing::info!("Found existing Google link for user_id: {}", user_id);
                // 更新用户的 Google 信息（可能昵称、头像有变化）
                self.sync_google_user_info(user_id, google_user).await?;
                return Ok(user_id);
            }
        }

        // 步骤 2: 通过邮箱匹配现有用户（账户合并场景）
        if let Some(user) = self.find_by_email(&google_user.email).await? {
            if let Some(user_id) = user.id {
                tracing::info!(
                    "Found email match for Google user {} -> user_id: {}",
                    google_user.email,
                    user_id
                );
                // 关联 Google 账户到已存在用户
                self.link_google_provider(user_id, google_user).await?;
                // 同步用户信息
                self.sync_google_user_info(user_id, google_user).await?;
                return Ok(user_id);
            }
        }

        // 步骤 3: 通过 Google 姓名匹配现有用户
        if let Some(google_name) = &google_user.name {
            // 首先尝试通过 github_name 字段匹配（可能用户之前用 GitHub 登录过）
            if let Some(user) = self
                .user_repository
                .find_by_github_name(google_name)
                .await?
            {
                if let Some(user_id) = user.id {
                    tracing::info!(
                        "Found GitHub name field match for Google user {} (name: {}) -> user_id: {}",
                        google_user.email,
                        google_name,
                        user_id
                    );
                    // 关联 Google 账户到已存在用户
                    self.link_google_provider(user_id, google_user).await?;
                    // 同步用户信息
                    self.sync_google_user_info(user_id, google_user).await?;
                    return Ok(user_id);
                }
            }

            // 其次尝试通过用户名匹配 Google 显示名称
            if let Some(user) = self.find_by_username(google_name).await? {
                if let Some(user_id) = user.id {
                    tracing::info!(
                        "Found username match for Google user {} (name: {}) -> user_id: {}",
                        google_user.email,
                        google_name,
                        user_id
                    );
                    // 关联 Google 账户到已存在用户
                    self.link_google_provider(user_id, google_user).await?;
                    // 同步用户信息
                    self.sync_google_user_info(user_id, google_user).await?;
                    return Ok(user_id);
                }
            }
        }

        // 步骤 4: 创建新用户（基于邮箱用户名部分）
        tracing::warn!(
            "No automatic match found for Google user {}. Creating new user. Consider manual linking if this is an existing user.",
            google_user.email
        );
        let base_username = google_user.email.split('@').next().unwrap_or("google_user");
        let username = self.generate_unique_username(base_username).await?;

        let email = &google_user.email;
        let nick_name = google_user.name.clone();

        // OAuth 用户使用随机密码（他们不会使用密码登录）
        let dummy_password = self.uuid_service.generate();

        let user_id = self
            .create_user(username, dummy_password, email.to_string(), nick_name)
            .await?;

        // 设置头像
        if let Some(picture_url) = &google_user.picture {
            if let Some(mut user) = self.find_by_id(user_id).await? {
                user.update_profile(user.nick_name.clone(), Some(picture_url.clone()));
                self.user_repository.update(&user).await?;
            }
        }

        // 关联 Google 提供商
        self.link_google_provider(user_id, google_user).await?;

        Ok(user_id)
    }

    /// 同步 Google 用户信息
    async fn sync_google_user_info(
        &self,
        user_id: i64,
        google_user: &GoogleUser,
    ) -> DomainResult<()> {
        if let Some(mut user) = self.find_by_id(user_id).await? {
            let mut updated = false;

            // 如果用户没有昵称，使用 Google 的名称
            if user.nick_name.is_none() && google_user.name.is_some() {
                user.update_profile(google_user.name.clone(), user.avatar.clone());
                updated = true;
            }

            // 如果用户没有头像，使用 Google 的头像
            if user.avatar.is_none() && google_user.picture.is_some() {
                user.update_profile(user.nick_name.clone(), google_user.picture.clone());
                updated = true;
            }

            if updated {
                self.user_repository.update(&user).await?;
                tracing::info!("Updated user {} profile from Google info", user_id);
            }
        }

        Ok(())
    }

    /// 关联 Google 提供商
    async fn link_google_provider(
        &self,
        user_id: i64,
        google_user: &GoogleUser,
    ) -> DomainResult<()> {
        let provider_raw =
            serde_json::to_value(google_user).map_err(|e| DomainError::InternalError {
                message: format!("JSON serialization failed: {}", e),
            })?;

        let provider = UserProvider::new(
            user_id,
            "google".to_string(),
            google_user.id.clone(),
            Some(google_user.email.clone()),
            Some(provider_raw),
        );

        self.provider_repository.save(&provider).await
    }

    /// 获取用户列表（分页）
    pub async fn get_user_list(
        &self,
        page: i32,
        page_size: i32,
        keyword: Option<String>,
        status: Option<String>,
    ) -> DomainResult<(Vec<User>, i64)> {
        self.user_repository
            .find_users_paginated(page, page_size, keyword, status)
            .await
    }

    /// 修改密码
    pub async fn change_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_password: &str,
    ) -> DomainResult<()> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound {
                resource: "用户".to_string(),
            })?;

        // 验证旧密码
        if !self.verify_password(old_password, &user.password_hash)? {
            return Err(DomainError::ValidationError {
                message: "原密码错误".to_string(),
            });
        }

        // 哈希新密码
        let new_password_hash = self.hash_password(new_password)?;
        user.password_hash = new_password_hash;

        self.user_repository.update(&user).await
    }
}
