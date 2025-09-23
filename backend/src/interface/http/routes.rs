// HTTP Routes

use axum::Router;
use std::sync::Arc;

use crate::application::database::DatabaseApplicationServiceImpl;
use crate::application::ddl_precheck::{
    DDLPrecheckApplicationService, DDLPrecheckApplicationServiceImpl,
};
use crate::application::notifications::NotificationsApplicationService;
use crate::application::services::EditorApplicationService;
use crate::application::settings::{SettingsApplicationService, SettingsApplicationServiceImpl};
use crate::application::slowlog::{SlowlogApplicationService, SlowlogApplicationServiceImpl};
use crate::application::sql_editor::SqlEditorApplicationServiceImpl;
use crate::application::table::{TableApplicationService, TableApplicationServiceImpl};
use crate::application::DatabaseApplicationService;
use crate::interface::http::database_controllers::{DatabaseController, DatabaseControllerState};
use crate::interface::http::ddl_controllers::{DDLPrecheckController, DDLPrecheckControllerState};
use crate::interface::http::extension_controllers::{ExtensionAppState, create_extension_routes};
use crate::interface::http::health_controllers::HealthController;
use crate::interface::http::notifications_controllers::{
    NotificationsAppState, NotificationsController,
};
use crate::interface::http::settings_controllers::{SettingsAppState, SettingsController};
use crate::interface::http::slowlog_controllers::{SlowlogAppState, SlowlogController};
use crate::interface::http::sql_editor_controllers::{
    SqlEditorController, SqlEditorControllerState,
};
use crate::interface::http::table_controllers::{TableAppState, TableController};

/// 创建所有 API 路由
/// 使用应用服务进行业务逻辑处理
pub fn create_api_routes() -> Router {
    // 创建应用服务实例
    let slowlog_service: Arc<dyn SlowlogApplicationService> =
        Arc::new(SlowlogApplicationServiceImpl::new());
    let ddl_service: Arc<dyn DDLPrecheckApplicationService> =
        Arc::new(DDLPrecheckApplicationServiceImpl::new());
    let settings_service: Arc<dyn SettingsApplicationService> =
        Arc::new(SettingsApplicationServiceImpl::new());
    let notifications_service = Arc::new(NotificationsApplicationService::new());
    let table_service: Arc<dyn TableApplicationService> =
        Arc::new(TableApplicationServiceImpl::new());
    let database_service: Arc<dyn DatabaseApplicationService> =
        Arc::new(DatabaseApplicationServiceImpl::new());
    let sql_editor_service: Arc<dyn EditorApplicationService> =
        Arc::new(SqlEditorApplicationServiceImpl::new());

    // 创建应用状态
    let slowlog_state = SlowlogAppState::new(slowlog_service);
    let ddl_state = DDLPrecheckControllerState::new(ddl_service);
    let settings_state = SettingsAppState::new(settings_service);
    let notifications_state = NotificationsAppState::new(notifications_service);
    let table_state = TableAppState::new(table_service);
    let database_state = DatabaseControllerState::new();
    let sql_editor_state = SqlEditorControllerState::new(sql_editor_service);
    
    // 创建扩展应用状态
    let extension_state = ExtensionAppState::new();

    // 组合所有路由
    Router::new()
        .merge(SlowlogController::routes().with_state(slowlog_state))
        .merge(DDLPrecheckController::routes().with_state(ddl_state))
        .merge(HealthController::routes())
        .merge(SettingsController::routes().with_state(settings_state))
        .merge(NotificationsController::routes().with_state(notifications_state))
        .merge(TableController::routes().with_state(table_state))
        .merge(DatabaseController::routes().with_state(database_state))
        .merge(SqlEditorController::routes().with_state(sql_editor_state))
        .nest("/api", create_extension_routes().with_state(extension_state))
}
