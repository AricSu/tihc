use crate::infrastructure::startup::InfraState as AppState;
use crate::interface::http::controllers::{
    auth_controller::{
        captcha_handler, github_oauth_callback_handler, github_oauth_start_handler,
        google_oauth_callback_handler, google_oauth_start_handler, login_handler, logout_handler,
    },
    chat_controller::{
        add_chat_message_handler, create_session_handler, get_chat_history_handler,
        sessions_handler, start_chat_handler, stream_chat_handler,
    },
    role_controller::permission_tree_handler,
    user_controller::{
        change_password_handler, get_user_detail_handler,get_user_list_handler
    },
};
use crate::interface::http::middleware::auth_middleware;
use axum::http::{HeaderName, HeaderValue, Method};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// 创建API路由 - 符合DDD原则：接口层只负责路由定义
/// AppState应该从外部传入，而不是在这里创建
pub fn create_api_routes(app_state: Arc<AppState>) -> Router {
    // Public routes (no authentication required)
    // 注意：这些路由不需要 JWT token，但有其他安全措施
    let public_routes = Router::new()
        // Authentication endpoints - 登录前必须的接口
        .route("/auth/login", post(login_handler)) // 需要验证码 + 密码哈希
        .route("/auth/captcha", get(captcha_handler)) // JSON格式验证码（统一接口）
        .route("/auth/oauth/github/start", get(github_oauth_start_handler)) // CSRF protection
        .route(
            "/auth/oauth/github/callback",
            get(github_oauth_callback_handler),
        ) // 授权码验证
        .route("/auth/oauth/google/start", get(google_oauth_start_handler)) // Google OAuth 开始
        .route(
            "/auth/oauth/google/callback",
            get(google_oauth_callback_handler),
        ) // Google OAuth 回调
        // System endpoints - 系统监控接口
        .route("/health", get(|| async { "OK" })); // 无敏感信息，可公开

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // Authentication routes (requires token)
        .route("/auth/logout", post(logout_handler))
        // User management routes
        .route("/user/detail", get(get_user_detail_handler))
        .route("/user/list", get(get_user_list_handler))
        .route("/user/password", post(change_password_handler))
        .route("/role/permissions/tree", get(permission_tree_handler))
        // Chat routes
        .route("/chat/start", post(start_chat_handler))
        .route("/chat/history", get(get_chat_history_handler))
        .route("/chat/history", post(add_chat_message_handler))
        .route("/chat/sessions", get(sessions_handler))
        .route("/chat/sessions", post(create_session_handler))
        .route("/chat/stream", get(stream_chat_handler))
        // TODO: Add more protected routes as needed:
        // .route("/user/profile", post(update_user_profile_handler))
        // .route("/user/password", post(change_password_handler))
        // SQL Editor routes (when implemented)
        // .route("/sql_editor/connections", get(list_connections_handler))
        // .route("/sql_editor/connections", post(create_connection_handler))
        // .route("/sql_editor/connections/:id", delete(delete_connection_handler))
        // .route("/sql_editor/execute", post(execute_sql_handler))
        // Inspection/Profile routes (when implemented)
        // .route("/inspection/tasks", get(list_inspection_tasks_handler))
        // .route("/inspection/tasks", post(create_inspection_task_handler))
        // .route("/inspection/tasks/:id/status", get(get_task_status_handler))
        // System settings routes (when implemented)
        // .route("/settings", get(get_settings_handler))
        // .route("/settings", post(update_settings_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    // Combine all routes with shared state
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3200".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3200".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
            HeaderName::from_static("x-requested-with"),
            HeaderName::from_static("accept"),
            HeaderName::from_static("origin"),
            HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true);

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state)
        .layer(cors)
}
