// Infrastructure 层：依赖注入容器
// 符合DDD原则：基础设施层负责具体实现的组装

use anyhow::Result;
use std::sync::Arc;

use crate::application::auth::{AuthService, OAuthService, UserService};
use crate::application::config::AppConfigService;
use crate::domain::auth::CaptchaService;
use crate::domain::auth::{AuthTokenStore, jwt::JwtService};
use crate::domain::basic::AppConfig;
use crate::infrastructure::database::{create_mysql_pool, run_migrations};
use crate::infrastructure::jwt::{BcryptPasswordService, JsonWebTokenService, UuidV4Service};
use crate::infrastructure::repositories::{
    AuthTokenRepository, ChatHistoryRepository, FileConfigRepository, InMemoryCaptchaRepository,
    MySqlMenuRepository, MySqlUserProviderRepository, MySqlUserRepository,
};

/// 具体服务类型定义
pub type ConcreteAuthService = AuthService<InMemoryCaptchaRepository>;
pub type ConcreteOAuthService = OAuthService<AppConfigService>;
pub type ConcreteUserService = UserService;
pub type ConcreteCaptchaService = CaptchaService<InMemoryCaptchaRepository>;

/// 依赖注入容器
pub struct DiContainer {
    pub auth_service: Arc<ConcreteAuthService>,
    pub oauth_service: Arc<ConcreteOAuthService>,
    pub user_service: Arc<ConcreteUserService>,
    pub menu_repo: Arc<MySqlMenuRepository>,
    pub chat_history_repo: Arc<ChatHistoryRepository>,
    pub auth_token_store: Arc<dyn AuthTokenStore>,
}

impl DiContainer {
    /// 创建完整的依赖图
    pub async fn new(config_value: &toml::Value) -> Result<Self> {
        // 通过 microkernel 传递的 toml::Value 构建 config
        let config = AppConfig::from_toml_value(config_value)?;

        // 创建数据库连接池
        let database_url = config.to_database_url();
        tracing::info!(
            "🗄️  Database DSN (with db): {}",
            database_url.replace(&config.database.password, "***")
        );

        // 为了能在数据库尚未创建时运行迁移，我们先使用不带数据库名的连接去创建数据库（如果不存在）
        let tls_param = if config.database.use_tls {
            "?ssl-mode=REQUIRED"
        } else {
            ""
        };
        let admin_url = format!(
            "mysql://{}:{}@{}:{}{}",
            config.database.username,
            config.database.password,
            config.database.host,
            config.database.port,
            tls_param
        );

        tracing::info!(
            "🗄️  Connecting to DB server for initial setup: {}",
            admin_url.replace(&config.database.password, "***")
        );
        let admin_pool = create_mysql_pool(&admin_url).await?;

        // 创建数据库（如果不存在），使用 utf8mb4 字符集
        let create_db_sql = format!(
            "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;",
            config.database.name
        );
        sqlx::query(&create_db_sql)
            .execute(&admin_pool)
            .await
            .map_err(|e| anyhow::anyhow!("failed to create database: {}", e))?;

        // 丢弃 admin_pool，后面用带数据库名的连接池继续
        drop(admin_pool);

        tracing::info!(
            "🗄️  Connecting to database: {}",
            database_url.replace(&config.database.password, "***")
        );
        let pool = create_mysql_pool(&database_url).await?;

        // 运行数据库迁移
        run_migrations(&pool).await?;

        // 配置仓储和服务
        let config_repository = Arc::new(FileConfigRepository::new(config_value.clone()));
        let config_service = Arc::new(AppConfigService::new(config_repository.clone()));

        // 仓储层 - 使用真实的MySQL仓储和内存验证码仓储
        let captcha_repo = InMemoryCaptchaRepository::new();
        let user_repo = Arc::new(MySqlUserRepository::new(pool.clone()));
        let provider_repo = Arc::new(MySqlUserProviderRepository::new(pool.clone()));
        let menu_repo = Arc::new(MySqlMenuRepository::new(pool.clone()));
        let chat_history_repo = Arc::new(ChatHistoryRepository::new(pool.clone()));
        let auth_token_repo = Arc::new(AuthTokenRepository::new(pool.clone()));
        let auth_token_store: Arc<dyn AuthTokenStore> = auth_token_repo.clone();

        // 基础服务层
        let password_service = Arc::new(BcryptPasswordService::new());
        let id_service = Arc::new(UuidV4Service::new());
        let jwt_service: Arc<dyn JwtService> =
            Arc::new(JsonWebTokenService::new(config_repository.clone()));

        // 应用服务层
        let user_service = Arc::new(UserService::new(
            user_repo.clone(),
            provider_repo.clone(),
            password_service.clone(),
            id_service.clone(),
        ));

        let oauth_service = Arc::new(OAuthService::new(config_service.clone()));

        let captcha_service = Arc::new(CaptchaService::new(
            captcha_repo,
            config.captcha.expiry_seconds,
        ));

        let auth_service = Arc::new(AuthService::new(
            user_service.clone(),
            jwt_service.clone(),
            captcha_service.clone(),
            auth_token_store.clone(),
        ));

        Ok(Self {
            auth_service,
            oauth_service,
            user_service,
            menu_repo,
            chat_history_repo,
            auth_token_store,
        })
    }
}
