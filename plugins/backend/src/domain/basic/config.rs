use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

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
    pub fn load() -> Result<Self, ConfigError> {
        // 尝试从多个位置加载配置文件
        let config_paths = ["_config.toml"];

        for path in &config_paths {
            if Path::new(path).exists() {
                tracing::info!("📄 Loading configuration from: {}", path);
                return Self::load_from_file(path);
            }
        }

        // 如果没有找到配置文件，创建默认配置
        tracing::warn!("⚠️  No configuration file found, using defaults");
        Ok(Self::default())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content =
            fs::read_to_string(path.as_ref()).map_err(|e| ConfigError::FileRead(e.to_string()))?;

        let config: AppConfig =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(config)
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
