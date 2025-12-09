use axum::{
    Json,
    extract::{Extension, Query, State},
    response::IntoResponse,
};
use base64::Engine;
use std::sync::Arc;

use crate::domain::auth::Claims;
use crate::{
    application::auth::{AuthErrorMapper, dtos::LoginRequest},
    infrastructure::InfraState as AppState,
    interface::http::ApiResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GitHubOAuthStartQuery {
    pub is_extension: Option<bool>,
    pub extension_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleOAuthStartQuery {}


#[derive(Debug, Serialize)]
pub struct CaptchaResponse {
    pub image_base64: String,
    pub session_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: Option<String>,
    pub error: Option<String>,
}
// 统一错误响应辅助函数
fn error_response(code: i32, msg: impl ToString) -> axum::response::Response {
    ApiResponse::<()>::error(code.try_into().unwrap(), msg.to_string()).into_response()
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    tracing::info!(target: "login_handler", "login_handler called");
    let auth_service = &app_state.auth_service;
    match auth_service.login(request).await {
        Ok(response) => ApiResponse::success(response).into_response(),
        Err(e) => {
            use crate::domain::shared::DomainError;
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            let status = match &e {
                DomainError::ValidationError { .. } => axum::http::StatusCode::BAD_REQUEST,
                DomainError::NotFound { .. } => axum::http::StatusCode::NOT_FOUND,
                DomainError::BusinessRuleViolation { .. } => axum::http::StatusCode::BAD_REQUEST,
                DomainError::AuthenticationError { .. } => axum::http::StatusCode::UNAUTHORIZED,
                DomainError::InternalError { .. } => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            tracing::debug!(target: "login_handler", "登录失败: {:?}, message: {}", e, message);
            axum::response::Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_string(&ApiResponse::<()>::error(code, message)).unwrap()
                ))
                .unwrap()
        }
    }
}

/// 返回JSON格式的验证码（统一接口）
pub async fn captcha_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;
    match auth_service.generate_captcha() {
        Ok(captcha_info) => {
            let base64_image =
                base64::engine::general_purpose::STANDARD.encode(&captcha_info.image_data);

            let response = CaptchaResponse {
                image_base64: format!("data:image/png;base64,{}", base64_image),
                session_id: captcha_info.session_id,
            };
            ApiResponse::success(response).into_response()
        }
        Err(e) => ApiResponse::<()>::error(10000, e.to_string()).into_response(),
    }
}

pub async fn github_oauth_start_handler(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<GitHubOAuthStartQuery>,
) -> impl IntoResponse {
    let oauth_service = &app_state.oauth_service;
    let is_extension = query.is_extension.unwrap_or(false);
    let extension_id = query.extension_id.clone();
    match oauth_service.generate_github_auth_url(None, is_extension, extension_id).await {
        Ok(response) => ApiResponse::success(response).into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("GitHub OAuth not configured")
                || error_msg.contains("demo_client_id")
            {
                let setup_guide = r#"GitHub OAuth 未正确配置。请按以下步骤设置：

1. 访问 https://github.com/settings/developers
2. 点击 "New OAuth App"
3. 填写应用信息：
   - Application name: TiHC Development
   - Homepage URL: http://localhost:5173
   - Authorization callback URL: http://127.0.0.1:8080/auth/oauth/github/callback
4. 创建后，将 Client ID 和 Client Secret 更新到 _config.toml 文件中的 [github_oauth] 部分
5. 重启服务器

当前配置使用的是演示用的无效凭据。"#;

                ApiResponse::<()>::error(400, setup_guide.to_string()).into_response()
            } else {
                ApiResponse::<()>::error(503, format!("OAuth configuration error: {}", e))
                    .into_response()
            }
        }
    }
}

pub async fn github_oauth_callback_handler(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;
    let oauth_service = &app_state.oauth_service;
    if let Some(error) = query.error {
        return error_response(400, format!("OAuth error: {}", error));
    }
    let code = match query.code {
        Some(code) => code,
        None => return error_response(400, "Missing authorization code"),
    };
    let access_token = match oauth_service.exchange_code_for_token(&code).await {
        Ok(token) => token,
        Err(e) => return error_response(500, format!("Failed to get access token: {}", e)),
    };
    let github_user = match oauth_service.get_github_user_info(&access_token).await {
        Ok(user) => user,
        Err(e) => return error_response(500, format!("Failed to get user info: {}", e)),
    };
    match auth_service.github_oauth_login(github_user).await {
        Ok(response) => ApiResponse::success(response).into_response(),
        Err(e) => error_response(500, e.to_string()),
    }
}

pub async fn google_oauth_start_handler(
    State(app_state): State<Arc<AppState>>,
    Query(_query): Query<GoogleOAuthStartQuery>,
) -> impl IntoResponse {
    let oauth_service = &app_state.oauth_service;
    match oauth_service.generate_google_auth_url(None).await {
        Ok(response) => ApiResponse::success(response).into_response(),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Google OAuth not configured")
                || error_msg.contains("demo_client_id")
            {
                let setup_guide = r#"Google OAuth 未正确配置。请按以下步骤设置：

1. 访问 https://console.cloud.google.com/
2. 创建新项目或选择现有项目
3. 启用 Google+ API
4. 创建 OAuth 2.0 客户端 ID：
   - 应用类型：Web 应用
   - 授权的重定向 URI: http://127.0.0.1:8080/auth/oauth/google/callback
5. 将 Client ID 和 Client Secret 更新到 _config.toml 文件中的 [google_oauth] 部分
6. 重启服务器

当前配置使用的是演示用的无效凭据。"#;

                ApiResponse::<()>::error(400, setup_guide.to_string()).into_response()
            } else {
                ApiResponse::<()>::error(503, format!("OAuth configuration error: {}", e))
                    .into_response()
            }
        }
    }
}

pub async fn google_oauth_callback_handler(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;
    let oauth_service = &app_state.oauth_service;
    if let Some(error) = query.error {
        return error_response(400, format!("OAuth error: {}", error));
    }
    let code = match query.code {
        Some(code) => code,
        None => return error_response(400, "Missing authorization code"),
    };
    let access_token = match oauth_service.exchange_google_code_for_token(&code).await {
        Ok(token) => token,
        Err(e) => return error_response(500, format!("Failed to get access token: {}", e)),
    };
    let google_user = match oauth_service.get_google_user_info(&access_token).await {
        Ok(user) => user,
        Err(e) => return error_response(500, format!("Failed to get user info: {}", e)),
    };
    match auth_service.google_oauth_login(google_user).await {
        Ok(response) => ApiResponse::success(response).into_response(),
        Err(e) => error_response(500, e.to_string()),
    }
}
/// 退出登录处理器
pub async fn logout_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Extension(token): Extension<String>,
) -> impl IntoResponse {
    if let Err(e) = app_state.auth_service.revoke_token(&token).await {
        tracing::warn!("Failed to revoke token in store: {}", e);
        return ApiResponse::<()>::error(500, "注销失败，请稍后重试".to_string()).into_response();
    }

    ApiResponse::success(true).into_response()
}
