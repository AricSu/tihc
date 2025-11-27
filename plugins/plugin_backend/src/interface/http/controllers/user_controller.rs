use crate::application::auth::dtos::{
    ChangePasswordRequest, CreateUserRequest, UpdateUserRequest, UserDetailResponse, UserListItem,
    UserListRequest, UserListResponse,
};
use crate::domain::auth::Claims;
use crate::domain::shared::DomainError;
use crate::{
    application::auth::AuthErrorMapper, infrastructure::InfraState as AppState,
    interface::http::ApiResponse,
};
use axum::{
    Json,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use validator::Validate;

pub async fn get_user_detail_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let auth_service = &app_state.auth_service;
    let user_id: i64 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user ID").into_response(),
    };

    match auth_service.get_user_by_id(user_id).await {
        Ok(user) => {
            let response = UserDetailResponse::from(user);
            ApiResponse::success(response).into_response()
        }
        Err(DomainError::NotFound { .. }) => {
            ApiResponse::<()>::error(404, "User not found".to_string()).into_response()
        }
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}

/// 获取用户列表处理器
pub async fn get_user_list_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Query(request): Query<UserListRequest>,
) -> impl IntoResponse {
    // 验证请求参数
    if let Err(e) = request.validate() {
        return ApiResponse::<()>::error(400, format!("参数验证失败: {:?}", e)).into_response();
    }

    let user_service = &app_state.user_service;

    match user_service
        .get_user_list(
            request.page,
            request.page_size,
            request.keyword,
            request.status,
        )
        .await
    {
        Ok((users, total)) => {
            // 转换为响应格式
            let list: Vec<UserListItem> = users
                .into_iter()
                .map(|user| UserListItem {
                    id: user.id.unwrap_or(0),
                    username: user.username,
                    email: user.email,
                    nick_name: user.nick_name,
                    avatar: user.avatar,
                    status: user.status,
                    created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    updated_at: user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                })
                .collect();

            let response = UserListResponse {
                list,
                total,
                page: request.page,
                page_size: request.page_size,
            };

            ApiResponse::success(response).into_response()
        }
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}

/// 创建用户处理器
pub async fn create_user_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // 验证请求参数
    if let Err(e) = request.validate() {
        return ApiResponse::<()>::error(400, format!("参数验证失败: {:?}", e)).into_response();
    }

    let user_service = &app_state.user_service;

    match user_service
        .create_user(
            request.username,
            request.password,
            request.email,
            request.nick_name,
        )
        .await
    {
        Ok(user_id) => ApiResponse::success(user_id).into_response(),
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}

/// 更新用户处理器
pub async fn update_user_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Path(user_id): Path<i64>,
    Json(request): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    // 验证请求参数
    if let Err(e) = request.validate() {
        return ApiResponse::<()>::error(400, format!("参数验证失败: {:?}", e)).into_response();
    }

    let user_service = &app_state.user_service;

    match user_service.update_user(user_id, request).await {
        Ok(()) => ApiResponse::success(true).into_response(),
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}

/// 删除用户处理器
pub async fn delete_user_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(_claims): Extension<Claims>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    let user_service = &app_state.user_service;

    match user_service.delete_user(user_id).await {
        Ok(()) => ApiResponse::success(true).into_response(),
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}

/// 修改密码处理器
pub async fn change_password_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    // 验证请求参数
    if let Err(e) = request.validate() {
        return ApiResponse::<()>::error(400, format!("参数验证失败: {:?}", e)).into_response();
    }

    let user_id: i64 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user ID").into_response(),
    };

    let user_service = &app_state.user_service;

    match user_service
        .change_password(user_id, &request.old_password, &request.new_password)
        .await
    {
        Ok(()) => ApiResponse::success(true).into_response(),
        Err(e) => {
            let code = AuthErrorMapper::map_error_code(&e);
            let message = AuthErrorMapper::get_user_message(&e);
            ApiResponse::<()>::error(code, message).into_response()
        }
    }
}
