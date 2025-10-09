// HTTP Controllers for Database Management
// 数据库管理的HTTP控制器 - 基于消息总线通信

use crate::application::services::{
    ColumnResponse, ConnectionResponse, CreateConnectionRequest, TableResponse,
    UpdateConnectionRequest,
};
use crate::infrastructure::bus_client::InfraBusClient;
use crate::interface::http::responses::ApiResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::Deserialize;
use serde_json::from_value;

#[derive(Debug, Clone)]
pub struct DatabaseControllerState {}

impl DatabaseControllerState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct DatabaseController;

impl DatabaseController {
    /// 创建数据库管理路由
    pub fn routes() -> Router<DatabaseControllerState> {
        Router::new()
            .route("/api/connections", post(create_connection))
            .route("/api/connections", get(list_connections))
            .route("/api/connections/{id}", get(get_connection))
            .route("/api/connections/{id}", put(update_connection))
            .route("/api/connections/{id}", delete(delete_connection))
            .route("/api/connections/test", post(test_connection))
            .route("/api/connections/{id}/tables", get(get_tables))
            .route(
                "/api/connections/{connection_id}/tables/{table_name}/columns",
                get(get_table_columns),
            )
    }
}

#[derive(serde::Serialize)]
pub struct ConnectionListResponse {
    pub connections: Vec<ConnectionResponse>,
}

#[derive(Deserialize)]
pub struct TestConnectionRequest {
    pub connection_id: String,
}

#[derive(serde::Serialize)]
pub struct TestConnectionResponse {
    pub success: bool,
}

#[derive(serde::Serialize)]
pub struct TableListResponse {
    pub tables: Vec<TableResponse>,
}

#[derive(serde::Serialize)]
pub struct ColumnListResponse {
    pub columns: Vec<ColumnResponse>,
}

/// 创建数据库连接 - 通过消息总线
pub async fn create_connection(
    State(_state): State<DatabaseControllerState>,
    Json(create_request): Json<CreateConnectionRequest>,
) -> Result<Json<ApiResponse<ConnectionResponse>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)>
{
    let bus = InfraBusClient::new();
    match bus
        .request(
            "sql_editor.add_connection",
            None,
            serde_json::to_value(create_request).map_err(|e| {
                let api_response =
                    ApiResponse::error(&format!("Failed to serialize request: {}", e), 500);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response));
            })?,
            None,
        )
        .await
    {
        Ok(value) => match from_value::<ConnectionResponse>(value) {
            Ok(connection) => Ok(Json(ApiResponse::success(connection))),
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 获取所有连接 - 通过消息总线
pub async fn list_connections(
    State(_state): State<DatabaseControllerState>,
) -> Result<
    Json<ApiResponse<ConnectionListResponse>>,
    (StatusCode, Json<ApiResponse<serde_json::Value>>),
> {
    let bus = InfraBusClient::new();
    match bus
        .request("sql_editor.list_connection", None, (), None)
        .await
    {
        Ok(value) => match from_value::<Vec<ConnectionResponse>>(value) {
            Ok(connections) => {
                let response_data = ConnectionListResponse { connections };
                Ok(Json(ApiResponse::success(response_data)))
            }
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 获取连接详情 - 通过消息总线
pub async fn get_connection(
    State(_state): State<DatabaseControllerState>,
    Path(connection_id): Path<String>,
) -> Result<Json<ApiResponse<ConnectionResponse>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)>
{
    let bus = InfraBusClient::new();
    match bus
        .request("sql_editor.get_connection", None, connection_id, None)
        .await
    {
        Ok(value) => match from_value::<ConnectionResponse>(value) {
            Ok(connection) => Ok(Json(ApiResponse::success(connection))),
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 更新连接 - 通过消息总线
pub async fn update_connection(
    State(_state): State<DatabaseControllerState>,
    Json(update_request): Json<UpdateConnectionRequest>,
) -> Result<Json<ApiResponse<ConnectionResponse>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)>
{
    let bus = InfraBusClient::new();
    let update_value = serde_json::to_value(update_request).map_err(|e| {
        let api_response = ApiResponse::error(&format!("Failed to serialize request: {}", e), 500);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response));
    })?;
    match bus
        .request("sql_editor.update_connection", None, update_value, None)
        .await
    {
        Ok(value) => match from_value::<ConnectionResponse>(value) {
            Ok(connection) => Ok(Json(ApiResponse::success(connection))),
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 删除连接 - 通过消息总线
pub async fn delete_connection(
    State(_state): State<DatabaseControllerState>,
    Path(connection_id): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)> {
    let bus = InfraBusClient::new();
    match bus
        .request("sql_editor.delete_connection", None, connection_id, None)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 测试连接 - 通过消息总线
pub async fn test_connection(
    State(_state): State<DatabaseControllerState>,
    Json(test_request): Json<TestConnectionRequest>,
) -> Result<
    Json<ApiResponse<TestConnectionResponse>>,
    (StatusCode, Json<ApiResponse<serde_json::Value>>),
> {
    let bus = InfraBusClient::new();
    match bus
        .request(
            "sql_editor.test_connection",
            None,
            test_request.connection_id.clone(),
            None,
        )
        .await
    {
        Ok(value) => match from_value::<bool>(value) {
            Ok(success) => {
                let test_response = TestConnectionResponse { success };
                Ok(Json(ApiResponse::success(test_response)))
            }
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 获取数据库表列表 - 通过消息总线
pub async fn get_tables(
    State(_state): State<DatabaseControllerState>,
    Path(connection_id): Path<String>,
) -> Result<Json<ApiResponse<TableListResponse>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)>
{
    let bus = InfraBusClient::new();
    match bus
        .request("sql_editor.get_tables", None, connection_id, None)
        .await
    {
        Ok(value) => match from_value::<Vec<TableResponse>>(value) {
            Ok(tables) => {
                let response_data = TableListResponse { tables };
                Ok(Json(ApiResponse::success(response_data)))
            }
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}

/// 获取表的列信息 - 通过消息总线
pub async fn get_table_columns(
    State(_state): State<DatabaseControllerState>,
    Path((connection_id, table_name)): Path<(String, String)>,
) -> Result<Json<ApiResponse<ColumnListResponse>>, (StatusCode, Json<ApiResponse<serde_json::Value>>)>
{
    #[derive(serde::Serialize)]
    struct TableColumnsRequest {
        connection_id: String,
        table_name: String,
    }

    let request = TableColumnsRequest {
        connection_id,
        table_name,
    };
    let bus = InfraBusClient::new();
    let request_value = serde_json::to_value(request).map_err(|e| {
        let api_response = ApiResponse::error(&format!("Failed to serialize request: {}", e), 500);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response));
    })?;
    match bus
        .request("sql_editor.get_table_columns", None, request_value, None)
        .await
    {
        Ok(value) => match from_value::<Vec<ColumnResponse>>(value) {
            Ok(columns) => {
                let response_data = ColumnListResponse { columns };
                Ok(Json(ApiResponse::success(response_data)))
            }
            Err(e) => {
                let api_response =
                    ApiResponse::error(&format!("Failed to parse response: {}", e), 500);
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
            }
        },
        Err(e) => {
            let api_response = ApiResponse::error(&format!("Message bus error: {}", e), 500);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(api_response)))
        }
    }
}
