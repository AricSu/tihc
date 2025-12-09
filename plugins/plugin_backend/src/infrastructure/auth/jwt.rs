use crate::domain::auth::{Claims, jwt::JwtService};
use crate::domain::{
    basic::ConfigRepository,
    shared::{
        DomainError, DomainResult,
        services::{PasswordService, UuidService},
    },
};
use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// bcrypt密码服务实现
pub struct BcryptPasswordService;

impl BcryptPasswordService {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordService for BcryptPasswordService {
    fn hash_password(&self, password: &str) -> DomainResult<String> {
        hash(password, DEFAULT_COST).map_err(|e| DomainError::InternalError {
            message: format!("Password hashing failed: {}", e),
        })
    }

    fn verify_password(&self, password: &str, hash: &str) -> DomainResult<bool> {
        verify(password, hash).map_err(|e| DomainError::InternalError {
            message: format!("Password verification failed: {}", e),
        })
    }
}

/// UUID v4 生成服务实现
pub struct UuidV4Service;

impl UuidV4Service {
    pub fn new() -> Self {
        Self
    }
}

impl UuidService for UuidV4Service {
    fn generate(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

/// JWT 服务的内部 Claims 结构（用于序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JwtClaims {
    pub sub: i64,
    pub username: String,
    pub email: String,
    pub nick_name: Option<String>,
    pub exp: usize,
}

/// JWT 服务实现
pub struct JsonWebTokenService {
    config_repository: Arc<dyn ConfigRepository>,
}

impl JsonWebTokenService {
    pub fn new(config_repository: Arc<dyn ConfigRepository>) -> Self {
        Self { config_repository }
    }
}

#[async_trait]
impl JwtService for JsonWebTokenService {
    async fn generate_token(
        &self,
        user_id: i64,
        username: String,
        email: String,
        nick_name: Option<String>,
    ) -> DomainResult<String> {
        let jwt_config = self.config_repository.get_jwt_config().await.map_err(|e| {
            DomainError::InternalError {
                message: format!("Failed to get JWT config: {}", e),
            }
        })?;

        let secret = jwt_config.secret;
        let expires_hours = jwt_config.expiry_hours as u64;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| {
            DomainError::InternalError {
                message: format!("Time error: {}", e),
            }
        })?;

        let exp = (now.as_secs() + (expires_hours * 3600)) as usize;

        let claims = JwtClaims {
            sub: user_id,
            username,
            email,
            nick_name,
            exp,
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        encode(&header, &claims, &encoding_key).map_err(|e| DomainError::InternalError {
            message: format!("JWT encoding failed: {}", e),
        })
    }

    async fn validate_token(&self, token: &str) -> DomainResult<Claims> {
        let jwt_config = self.config_repository.get_jwt_config().await.map_err(|e| {
            DomainError::InternalError {
                message: format!("Failed to get JWT config: {}", e),
            }
        })?;

        let secret = jwt_config.secret;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation).map_err(|e| {
            DomainError::AuthenticationError {
                message: format!("JWT verification failed: {}", e),
            }
        })?;

        Ok(Claims {
            sub: token_data.claims.sub.to_string(),
            username: token_data.claims.username,
            email: token_data.claims.email,
            nick_name: token_data.claims.nick_name,
            exp: token_data.claims.exp,
            iat: 0,                                // We'll set this when creating the token
            jti: uuid::Uuid::new_v4().to_string(), // Generate a new JTI for validation
        })
    }
}
