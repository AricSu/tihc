use crate::domain::shared::DomainResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// JWT服务接口：定义令牌管理契约
#[async_trait]
pub trait JwtService: Send + Sync {
    /// 生成JWT令牌
    async fn generate_token(
        &self,
        user_id: i64,
        username: String,
        email: String,
        nick_name: Option<String>,
    ) -> DomainResult<String>;

    /// 验证JWT令牌
    async fn validate_token(&self, token: &str) -> DomainResult<Claims>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub exp: i64,    // JWT expiration timestamp
    pub iat: i64,    // JWT issued at timestamp
    pub jti: String, // JWT ID for session management
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id as string
    pub username: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub exp: usize,  // expiration time
    pub iat: usize,  // issued at
    pub jti: String, // JWT ID
}

use crate::domain::auth::UserInfo;

impl From<Session> for Claims {
    fn from(session: Session) -> Self {
        Self {
            sub: session.user_id.to_string(),
            username: session.username,
            email: session.email,
            nick_name: session.nick_name,
            exp: session.exp as usize,
            iat: session.iat as usize,
            jti: session.jti,
        }
    }
}

impl From<Claims> for Session {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub.parse().unwrap_or(0),
            username: claims.username,
            email: claims.email,
            nick_name: claims.nick_name,
            exp: claims.exp as i64,
            iat: claims.iat as i64,
            jti: claims.jti,
        }
    }
}
