use async_trait::async_trait;
use std::error::Error;

use crate::domain::basic::{
    AppConfig, ConfigRepository, GitHubOAuthConfig, GoogleOAuthConfig, JwtConfig,
};

pub struct FileConfigRepository {
    config_value: toml::Value,
}

impl FileConfigRepository {
    pub fn new(config_value: toml::Value) -> Self {
        Self { config_value }
    }
}

#[async_trait]
impl ConfigRepository for FileConfigRepository {
    async fn get_github_oauth_config(
        &self,
    ) -> Result<Option<GitHubOAuthConfig>, Box<dyn Error + Send + Sync>> {
        let config = AppConfig::from_toml_value(&self.config_value)?;
        if let Some(oauth) = config.github_oauth {
            Ok(Some(GitHubOAuthConfig {
                client_id: oauth.client_id,
                client_secret: oauth.client_secret,
                redirect_uri: oauth.redirect_uri,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_google_oauth_config(
        &self,
    ) -> Result<Option<GoogleOAuthConfig>, Box<dyn Error + Send + Sync>> {
        let config = AppConfig::from_toml_value(&self.config_value)?;
        if let Some(oauth) = config.google_oauth {
            Ok(Some(GoogleOAuthConfig {
                client_id: oauth.client_id,
                client_secret: oauth.client_secret,
                redirect_uri: oauth.redirect_uri,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_jwt_config(&self) -> Result<JwtConfig, Box<dyn Error + Send + Sync>> {
        let config = AppConfig::from_toml_value(&self.config_value)?;
        Ok(JwtConfig {
            secret: config.jwt.secret,
            expiry_hours: config.jwt.expiry_hours,
        })
    }
}
