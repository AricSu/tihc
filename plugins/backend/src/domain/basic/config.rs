use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use toml::Value;

/// 配置值对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// Google OAuth 配置值对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// 配置仓储接口：定义配置访问契约
#[async_trait]
pub trait ConfigRepository: Send + Sync {
    /// 获取GitHub OAuth配置
    async fn get_github_oauth_config(
        &self,
    ) -> Result<Option<GitHubOAuthConfig>, Box<dyn Error + Send + Sync>>;

    /// 获取Google OAuth配置
    async fn get_google_oauth_config(
        &self,
    ) -> Result<Option<GoogleOAuthConfig>, Box<dyn Error + Send + Sync>>;

    /// 获取JWT配置
    async fn get_jwt_config(&self) -> Result<JwtConfig, Box<dyn Error + Send + Sync>>;
}

/// JWT配置值对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry_hours: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    pub max_connections: Option<u32>,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaConfig {
    pub expiry_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub github_oauth: Option<GitHubOAuthConfig>,
    pub google_oauth: Option<GoogleOAuthConfig>,
    pub captcha: CaptchaConfig,
    pub log: LogConfig,
}

impl AppConfig {
    pub fn from_toml_value(config: &Value) -> Result<Self, ConfigError> {
        // 只解析 plugins.backend 下的配置
        let db_cfg = config.get("database").ok_or_else(|| ConfigError::ParseError("Missing [plugins.backend.database] section".to_string()))?;
        let host = db_cfg.get("host").and_then(Value::as_str).unwrap_or("127.0.0.1").to_string();
        let port = db_cfg.get("port").and_then(Value::as_integer).unwrap_or(3306) as u16;
        let username = db_cfg.get("username").and_then(Value::as_str).unwrap_or("root").to_string();
        let password = db_cfg.get("password").and_then(Value::as_str).unwrap_or("").to_string();
        let name = db_cfg.get("name").and_then(Value::as_str).unwrap_or("tihc").to_string();
        let max_connections = db_cfg.get("max_connections").and_then(Value::as_integer).map(|v| v as u32);
        let use_tls = db_cfg.get("use_tls").and_then(Value::as_bool).unwrap_or(false);

        let database = DatabaseConfig {
            host,
            port,
            username,
            password,
            name,
            max_connections,
            use_tls,
        };

        // 其他配置项可按需解析
        // 这里只解析 database，其他如 jwt、captcha、oauth 可扩展

        Ok(Self {
            database,
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            jwt: JwtConfig {
                secret: "dev-jwt-secret-change-in-production".to_string(),
                expiry_hours: 24,
            },
            github_oauth: None,
            google_oauth: None,
            captcha: CaptchaConfig {
                expiry_seconds: 300,
            },
            log: LogConfig {
                level: "info".to_string(),
                style: "always".to_string(),
            },
        })
    }

    pub fn to_database_url(&self) -> String {
        let tls_param = if self.database.use_tls {
            "?ssl-mode=REQUIRED"
        } else {
            ""
        };
        format!(
            "mysql://{}:{}@{}:{}/{}{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name,
            tls_param
        )
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "127.0.0.1".to_string(),
                port: 3306,
                username: "root".to_string(),
                password: "password".to_string(),
                name: "tihc".to_string(),
                max_connections: Some(20),
                use_tls: false,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            jwt: JwtConfig {
                secret: "dev-jwt-secret-change-in-production".to_string(),
                expiry_hours: 24,
            },
            github_oauth: None,
            google_oauth: None,
            captcha: CaptchaConfig {
                expiry_seconds: 300,
            },
            log: LogConfig {
                level: "info".to_string(),
                style: "always".to_string(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(String),
    #[error("Failed to parse config: {0}")]
    ParseError(String),
}
