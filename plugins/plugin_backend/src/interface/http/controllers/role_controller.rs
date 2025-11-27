use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;

use crate::infrastructure::InfraState as AppState;
use crate::interface::http::response::ApiResponse;

/// 返回角色相关的权限树以及该角色已分配的权限 ID 列表
pub async fn permission_tree_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    // 始终返回权限树的数组形态（legacy）：前端当前不传 roleId，所以只返回 tree
    // 优先从 DB 的 `tihc_menu_tree.default_tree` 读取，若不存在则回退到 RoleService 的静态 tree
    let role_service = crate::application::auth::role_service::RoleService::new();

    match app_state.menu_repo.get_menu_tree("default_tree").await {
        Ok(Some(json_tree)) => ApiResponse::success(json_tree).into_response(),
        Ok(None) => match role_service.get_permission_tree(None).await {
            Ok((tree, _assigned)) => {
                let data = serde_json::json!(tree);
                ApiResponse::success(data).into_response()
            }
            Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
        },
        Err(e) => ApiResponse::<()>::error(500, e.to_string()).into_response(),
    }
}
