use std::sync::Arc;
use validator::Validate;

use crate::application::auth::UserService;
use crate::application::auth::dtos::LoginRequest;
use crate::domain::auth::{
    CaptchaService, GitHubUser, GoogleUser, LoginResponse, UserInfo, jwt::JwtService,
};
use crate::domain::shared::{DomainError, DomainResult};
use crate::infrastructure::CaptchaRepository;
use chrono::{DateTime, Utc};

pub struct AuthService<R: CaptchaRepository, T: crate::domain::auth::token::TokenRepository + ?Sized> {
    user_service: Arc<UserService>,
    jwt_service: Arc<dyn JwtService>,
    captcha_service: Arc<CaptchaService<R>>,
    token_service: Arc<crate::application::auth::token_service::TokenService<T>>,
}

impl<R: CaptchaRepository, T: crate::domain::auth::token::TokenRepository + ?Sized> AuthService<R, T> {
    pub fn new(
        user_service: Arc<UserService>,
        jwt_service: Arc<dyn JwtService>,
        captcha_service: Arc<CaptchaService<R>>,
        token_service: Arc<crate::application::auth::token_service::TokenService<T>>,
    ) -> Self {
        Self {
            user_service,
            jwt_service,
            captcha_service,
            token_service,
        }
    }

    /// 用户登录业务用例
    pub async fn login(&self, request: LoginRequest) -> DomainResult<LoginResponse> {
        // 验证输入
        request
            .validate()
            .map_err(|e| crate::domain::shared::DomainError::ValidationError {
                message: format!("输入验证失败: {:?}", e),
            })?;

        // 验证验证码
        if !self
            .captcha_service
            .validate_captcha(&request.captcha_session_id, &request.captcha)
        {
            self.captcha_service
                .remove_captcha(&request.captcha_session_id);
            return Err(crate::domain::shared::DomainError::ValidationError {
                message: "验证码错误".to_string(),
            });
        }
        self.captcha_service
            .remove_captcha(&request.captcha_session_id);

        // 查找用户
        let user = self
            .user_service
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: "用户".to_string(),
            })?;

        // 验证密码
        let is_valid = self
            .user_service
            .verify_password(&request.password, &user.password_hash)?;

        if !is_valid {
            return Err(crate::domain::shared::DomainError::ValidationError {
                message: "邮箱或密码错误".to_string(),
            });
        }

        // 检查用户状态
        if !user.is_available() {
            return Err(crate::domain::shared::DomainError::BusinessRuleViolation {
                rule: "用户账户不可用".to_string(),
            });
        }

        // 生成 JWT 令牌
        let user_id = user
            .id
            .ok_or_else(|| crate::domain::shared::DomainError::InternalError {
                message: "用户 ID 缺失".to_string(),
            })?;

        // 单点登录：撤销所有旧 token
        self.token_service.revoke_all_tokens_for_user(user_id).await?;

        let token = self
            .jwt_service
            .generate_token(
                user_id,
                user.username.clone(),
                user.email.clone(),
                user.nick_name.clone(),
            )
            .await
            .map_err(|e| crate::domain::shared::DomainError::InternalError {
                message: format!("令牌生成失败: {}", e),
            })?;

        let expires_in = self.persist_token(user_id, &token).await?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserInfo::from(user),
        })
    }

    /// 生成验证码
    pub fn generate_captcha(&self) -> DomainResult<crate::domain::auth::CaptchaInfo> {
        self.captcha_service.generate_captcha().map_err(|e| {
            crate::domain::shared::DomainError::InternalError {
                message: format!("验证码生成失败: {}", e),
            }
        })
    }

    /// 根据用户 ID 获取用户信息
    pub async fn get_user_by_id(&self, user_id: i64) -> DomainResult<UserInfo> {
        let user = self
            .user_service
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: "用户".to_string(),
            })?;

        Ok(UserInfo::from(user))
    }

    /// 验证令牌
    pub async fn validate_token(&self, token: &str) -> DomainResult<crate::domain::auth::Claims> {
        self.jwt_service.validate_token(token).await.map_err(|e| {
            crate::domain::shared::DomainError::ValidationError {
                message: format!("令牌验证失败: {}", e),
            }
        })
    }

    /// GitHub OAuth 登录
    pub async fn github_oauth_login(&self, github_user: GitHubUser) -> DomainResult<LoginResponse> {
        // 创建或更新 GitHub 用户
        let user_id = self
            .user_service
            .create_or_update_github_user(&github_user)
            .await?;

        // 获取用户信息
        let user = self
            .user_service
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: "用户".to_string(),
            })?;

        // 生成 JWT 令牌
        let token = self
            .jwt_service
            .generate_token(
                user_id,
                user.username.clone(),
                user.email.clone(),
                user.nick_name.clone(),
            )
            .await
            .map_err(|e| crate::domain::shared::DomainError::InternalError {
                message: format!("令牌生成失败: {}", e),
            })?;

        let expires_in = self.persist_token(user_id, &token).await?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserInfo::from(user),
        })
    }

    /// Google OAuth 登录
    pub async fn google_oauth_login(&self, google_user: GoogleUser) -> DomainResult<LoginResponse> {
        // 创建或更新 Google 用户
        let user_id = self
            .user_service
            .create_or_update_google_user(&google_user)
            .await?;

        // 获取用户信息
        let user = self
            .user_service
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| crate::domain::shared::DomainError::NotFound {
                resource: "用户".to_string(),
            })?;

        // 生成 JWT 令牌
        let token = self
            .jwt_service
            .generate_token(
                user_id,
                user.username.clone(),
                user.email.clone(),
                user.nick_name.clone(),
            )
            .await
            .map_err(|e| crate::domain::shared::DomainError::InternalError {
                message: format!("令牌生成失败: {}", e),
            })?;

        let expires_in = self.persist_token(user_id, &token).await?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserInfo::from(user),
        })
    }

    pub async fn revoke_token(&self, token: &str) -> DomainResult<()> {
        self.token_service
            .revoke_token(token)
            .await
            .map_err(|e| DomainError::InternalError {
                message: format!("注销令牌失败: {}", e),
            })
    }

    async fn persist_token(&self, user_id: i64, token: &str) -> DomainResult<i64> {
        let claims = self.jwt_service.validate_token(token).await.map_err(|e| {
            DomainError::InternalError {
                message: format!("令牌验证失败: {}", e),
            }
        })?;

        let expires_at = Self::exp_to_datetime(claims.exp)?;
        self.token_service
            .store_token(user_id, token, expires_at)
            .await
            .map_err(|e| DomainError::InternalError {
                message: format!("保存令牌失败: {}", e),
            })?;

        let remaining = expires_at.timestamp() - Utc::now().timestamp();
        Ok(remaining.max(0))
    }

    fn exp_to_datetime(exp: usize) -> DomainResult<DateTime<Utc>> {
        let exp = exp as i64;
        DateTime::from_timestamp(exp, 0).ok_or_else(|| DomainError::InternalError {
            message: format!("无法解析过期时间戳: {}", exp),
        })
    }
}
