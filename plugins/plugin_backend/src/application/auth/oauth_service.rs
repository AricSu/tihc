use crate::application::config::ConfigService;
use crate::domain::auth::{GitHubUser, GoogleUser};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct GitHubOAuthStartResponse {
    pub authorize_url: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct GoogleOAuthStartResponse {
    pub authorize_url: String,
    pub state: String,
}

/// OAuth 应用服务 - 使用标准 oauth2 crate
pub struct OAuthService<C: ConfigService> {
    config_service: Arc<C>,
}

impl<C: ConfigService> OAuthService<C> {
    pub fn new(config_service: Arc<C>) -> Self {
        Self { config_service }
    }

    /// 生成 GitHub OAuth 授权 URL
    pub async fn generate_github_auth_url(
        &self,
        custom_state: Option<String>,
        is_extension: bool,
        extension_id: Option<String>,
    ) -> Result<GitHubOAuthStartResponse, Box<dyn Error + Send + Sync>> {
        let config = self
            .config_service
            .get_github_oauth_config()
            .await?
            .ok_or("GitHub OAuth not configured")?;

        // 检查是否是演示配置
        if config.client_id.starts_with("demo_")
            || config.client_id == "your_github_client_id"
            || config.client_id == "YOUR_GITHUB_CLIENT_ID_HERE"
            || config.client_id.contains("your_")
            || config.client_id.contains("YOUR_")
        {
            return Err("GitHub OAuth not properly configured. Please set real client_id and client_secret in _config.toml".into());
        }
        let redirect_uri = if is_extension {
            if let Some(ext_id) = extension_id {
                format!("https://{}.chromiumapp.org/", ext_id)
            } else {
                return Err("extension_id required for extension OAuth".into());
            }
        } else {
            config.redirect_uri.clone()
        };
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

        let client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(oauth2::RedirectUrl::new(redirect_uri)?);

        let (auth_url, csrf_token) = if let Some(state) = custom_state {
            client
                .authorize_url(|| CsrfToken::new(state.clone()))
                .add_scope(Scope::new("user:email".to_string()))
                .url()
        } else {
            client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("user:email".to_string()))
                .url()
        };

        Ok(GitHubOAuthStartResponse {
            authorize_url: auth_url.to_string(),
            state: csrf_token.secret().clone(),
        })
    }

    /// 用授权码换取访问令牌
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let config = self
            .config_service
            .get_github_oauth_config()
            .await?
            .ok_or("GitHub OAuth not configured")?;
        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;

        let client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url);

        let http_client = reqwest::Client::new();
        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&http_client)
            .await?;

        Ok(token_result.access_token().secret().clone())
    }

    /// 获取 GitHub 用户信息
    pub async fn get_github_user_info(
        &self,
        access_token: &str,
    ) -> Result<GitHubUser, Box<dyn Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "tihc-app")
            .send()
            .await?;

        if response.status().is_success() {
            let github_user: GitHubUser = response.json().await?;
            Ok(github_user)
        } else {
            Err(format!("GitHub API returned status: {}", response.status()).into())
        }
    }

    /// 生成 Google OAuth 授权 URL
    pub async fn generate_google_auth_url(
        &self,
        custom_state: Option<String>,
    ) -> Result<GoogleOAuthStartResponse, Box<dyn Error + Send + Sync>> {
        let config = self
            .config_service
            .get_google_oauth_config()
            .await?
            .ok_or("Google OAuth not configured")?;

        // 检查是否是演示配置
        if config.client_id.starts_with("demo_")
            || config.client_id == "your_google_client_id"
            || config.client_id == "YOUR_GOOGLE_CLIENT_ID_HERE"
            || config.client_id.contains("your_")
            || config.client_id.contains("YOUR_")
        {
            return Err("Google OAuth not properly configured. Please set real client_id and client_secret in _config.toml".into());
        }

        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;

        let client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(oauth2::RedirectUrl::new(config.redirect_uri.clone())?);

        let (auth_url, csrf_token) = if let Some(state) = custom_state {
            client
                .authorize_url(|| CsrfToken::new(state.clone()))
                .add_scope(Scope::new("openid".to_string()))
                .add_scope(Scope::new("email".to_string()))
                .add_scope(Scope::new("profile".to_string()))
                .url()
        } else {
            client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("openid".to_string()))
                .add_scope(Scope::new("email".to_string()))
                .add_scope(Scope::new("profile".to_string()))
                .url()
        };

        Ok(GoogleOAuthStartResponse {
            authorize_url: auth_url.to_string(),
            state: csrf_token.secret().clone(),
        })
    }

    /// 用授权码换取 Google 访问令牌
    pub async fn exchange_google_code_for_token(
        &self,
        code: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let config = self
            .config_service
            .get_google_oauth_config()
            .await?
            .ok_or("Google OAuth not configured")?;
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;

        let client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(oauth2::RedirectUrl::new(config.redirect_uri.clone())?);

        let http_client = reqwest::Client::new();
        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&http_client)
            .await?;

        Ok(token_result.access_token().secret().clone())
    }

    /// 获取 Google 用户信息
    pub async fn get_google_user_info(
        &self,
        access_token: &str,
    ) -> Result<GoogleUser, Box<dyn Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let google_user: GoogleUser = response.json().await?;
            Ok(google_user)
        } else {
            Err(format!("Google API returned status: {}", response.status()).into())
        }
    }
}
