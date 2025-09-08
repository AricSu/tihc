// Settings HTTP Controllers

use axum::{http::StatusCode, Json, Router, routing::{get, post}};
use std::sync::Arc;

use crate::application::settings::{
    SettingsApplicationService, SetLanguageRequest, LanguageResponse
};
use crate::interface::http::responses::ApiResponse;

pub struct SettingsController;

impl SettingsController {
    /// 创建设置相关路由
    pub fn routes() -> Router<SettingsAppState> {
        Router::new()
            .route("/lang", get(get_language_handler))
            .route("/lang", post(set_language_handler))
            .route("/lang/supported", get(get_supported_languages_handler))
    }
}

/// 设置应用状态
#[derive(Clone)]
pub struct SettingsAppState {
    pub settings_service: Arc<dyn SettingsApplicationService>,
}

impl SettingsAppState {
    pub fn new(settings_service: Arc<dyn SettingsApplicationService>) -> Self {
        Self { settings_service }
    }
}

/// 获取当前语言设置
async fn get_language_handler(
    axum::extract::State(state): axum::extract::State<SettingsAppState>,
) -> Result<Json<ApiResponse<LanguageResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    match state.settings_service.get_language().await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::empty_error(&e.to_string(), 500))
        )),
    }
}

/// 设置语言
async fn set_language_handler(
    axum::extract::State(state): axum::extract::State<SettingsAppState>,
    Json(request): Json<SetLanguageRequest>,
) -> Result<StatusCode, (StatusCode, Json<ApiResponse<()>>)> {
    match state.settings_service.set_language(request).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::empty_error(&e.to_string(), 400))
        )),
    }
}

/// 获取支持的语言列表
async fn get_supported_languages_handler(
    axum::extract::State(state): axum::extract::State<SettingsAppState>,
) -> Result<Json<ApiResponse<Vec<String>>>, (StatusCode, Json<ApiResponse<()>>)> {
    match state.settings_service.get_supported_languages().await {
        Ok(languages) => Ok(Json(ApiResponse::success(languages))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::empty_error(&e.to_string(), 500))
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;
    use crate::application::settings::SettingsApplicationServiceImpl;

    #[tokio::test]
    async fn test_get_language_endpoint() {
        let settings_service = Arc::new(SettingsApplicationServiceImpl::new());
        let state = SettingsAppState::new(settings_service);
        let app = SettingsController::routes().with_state(state);

        let request = Request::builder()
            .uri("/lang")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_supported_languages_endpoint() {
        let settings_service = Arc::new(SettingsApplicationServiceImpl::new());
        let state = SettingsAppState::new(settings_service);
        let app = SettingsController::routes().with_state(state);

        let request = Request::builder()
            .uri("/lang/supported")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
