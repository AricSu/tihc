use axum::{extract::State, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

use crate::application::extension::ExtensionApplicationService;
use crate::domain::extension::CollectionRequest;
use common::json_response::{JsonResponse, RespStatus};

/// 扩展数据采集控制器
pub struct ExtensionController {
    extension_service: Arc<dyn ExtensionApplicationService>,
}

impl ExtensionController {
    pub fn new(extension_service: Arc<dyn ExtensionApplicationService>) -> Self {
        Self { extension_service }
    }
    
    /// 创建扩展相关的路由
    pub fn routes() -> Router<Arc<ExtensionController>> {
        Router::new()
            .route("/api/collect", post(collect_data))
            .route("/api/extension/stats", get(get_collection_stats))
            .route("/api/extension/clear", post(clear_collection_data))
    }
}

/// 数据采集请求的 HTTP 载荷
#[derive(Debug, Deserialize)]
pub struct CollectDataPayload {
    pub url: String,
    pub domain: String,
    pub cookies: String,
    pub page_type: String,
    pub user_agent: String,
    pub local_storage: Option<std::collections::HashMap<String, String>>,
    pub session_storage: Option<std::collections::HashMap<String, String>>,
    pub timestamp: i64,
    pub task_id: Option<String>,
}

/// 数据采集响应
#[derive(Debug, Serialize)]
pub struct CollectDataResponse {
    pub success: bool,
    pub message: String,
    pub timestamp: i64,
}

/// 处理扩展数据采集请求
pub async fn collect_data(
    State(controller): State<Arc<ExtensionController>>,
    Json(payload): Json<CollectDataPayload>,
) -> Result<Json<JsonResponse<CollectDataResponse>>, (StatusCode, Json<JsonResponse<String>>)> {
    info!(
        "收到扩展数据采集请求 - 域名: {}, URL: {}, 页面类型: {}, Cookies长度: {}",
        payload.domain,
        payload.url,
        payload.page_type,
        payload.cookies.len()
    );

    // 构造领域模型
    let collected_data = crate::domain::extension::CollectedData {
        cookies: payload.cookies,
        page_type: payload.page_type,
        user_agent: payload.user_agent,
        local_storage: payload.local_storage,
        session_storage: payload.session_storage,
    };

    let collection_request = CollectionRequest {
        url: payload.url.clone(),
        domain: payload.domain.clone(),
        timestamp: payload.timestamp,
        task_id: payload.task_id,
        data: collected_data,
    };

    // 调用应用服务处理采集请求
    match controller
        .extension_service
        .handle_collection(collection_request)
        .await
    {
        Ok(_) => {
            let response = CollectDataResponse {
                success: true,
                message: "数据采集成功".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            };

            info!("扩展数据采集成功 - 域名: {}", payload.domain);

            Ok(Json(JsonResponse::success(response)))
        }
        Err(e) => {
            error!(
                "扩展数据采集失败 - 域名: {}, 错误: {}",
                payload.domain, e
            );

            let error_response = format!("数据采集失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonResponse::status_msg(RespStatus::Failed, error_response)),
            ))
        }
    }
}

/// 获取采集统计信息
pub async fn get_collection_stats(
    State(controller): State<Arc<ExtensionController>>,
) -> Result<Json<JsonResponse<serde_json::Value>>, (StatusCode, Json<JsonResponse<String>>)> {
    info!("收到获取采集统计请求");

    match controller
        .extension_service
        .get_all_auth_info()
        .await
    {
        Ok(auth_infos) => {
            let stats = serde_json::json!({
                "total_domains": auth_infos.len(),
                "domains": auth_infos.iter().map(|info| &info.domain).collect::<Vec<_>>(),
                "last_collection": auth_infos.iter().map(|info| info.timestamp).max(),
                "by_page_type": {
                    "grafana": auth_infos.iter().filter(|info| matches!(info.page_type, crate::domain::extension::PageType::Grafana)).count(),
                    "clinic": auth_infos.iter().filter(|info| matches!(info.page_type, crate::domain::extension::PageType::Clinic)).count(),
                    "unknown": auth_infos.iter().filter(|info| matches!(info.page_type, crate::domain::extension::PageType::Unknown)).count(),
                }
            });
            
            info!("采集统计获取成功: {} 个域名", auth_infos.len());
            Ok(Json(JsonResponse::success(stats)))
        }
        Err(e) => {
            error!("获取采集统计失败: {}", e);
            let error_response = format!("获取统计失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonResponse::status_msg(RespStatus::Failed, error_response)),
            ))
        }
    }
}

/// 清除采集数据
pub async fn clear_collection_data(
    State(controller): State<Arc<ExtensionController>>,
) -> Result<Json<JsonResponse<String>>, (StatusCode, Json<JsonResponse<String>>)> {
    info!("收到清除采集数据请求");

    match controller.extension_service.clear_all_data().await {
        Ok(_) => {
            info!("采集数据清除成功");
            Ok(Json(JsonResponse::success("数据清除成功".to_string())))
        }
        Err(e) => {
            error!("清除采集数据失败: {}", e);
            let error_response = format!("清除数据失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JsonResponse::status_msg(RespStatus::Failed, error_response)),
            ))
        }
    }
}