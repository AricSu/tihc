// Application 层：配置应用服务
// 符合 DDD 原则：应用层通过接口依赖领域层，不直接依赖基础设施层

use std::error::Error;
use std::sync::Arc;

use crate::domain::basic::{ConfigRepository, GitHubOAuthConfig, GoogleOAuthConfig};

/// 配置应用服务接口
pub trait ConfigService: Send + Sync {
    async fn get_github_oauth_config(
        &self,
    ) -> Result<Option<GitHubOAuthConfig>, Box<dyn Error + Send + Sync>>;
    async fn get_google_oauth_config(
        &self,
    ) -> Result<Option<GoogleOAuthConfig>, Box<dyn Error + Send + Sync>>;
}

/// 配置应用服务实现
pub struct AppConfigService {
    config_repository: Arc<dyn ConfigRepository>,
}

impl AppConfigService {
    pub fn new(config_repository: Arc<dyn ConfigRepository>) -> Self {
        Self { config_repository }
    }
}

impl ConfigService for AppConfigService {
    async fn get_github_oauth_config(
        &self,
    ) -> Result<Option<GitHubOAuthConfig>, Box<dyn Error + Send + Sync>> {
        self.config_repository.get_github_oauth_config().await
    }

    async fn get_google_oauth_config(
        &self,
    ) -> Result<Option<GoogleOAuthConfig>, Box<dyn Error + Send + Sync>> {
        self.config_repository.get_google_oauth_config().await
    }
}
