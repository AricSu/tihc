// 扩展数据控制器 - 处理来自浏览器扩展的请求
use axum::{
    extract::{Json, State},
    http::{StatusCode, header::{CONTENT_TYPE, CONTENT_DISPOSITION}},
    response::{Json as ResponseJson, Response},
    routing::{get, post},
    Router,
    body::Body,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};

// 扩展数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionTokenData {
    pub session_id: Option<String>,
    pub csrf_token: Option<String>,
    pub apikey: Option<String>,
    pub cookie: Option<String>,
    pub relevant_cookies: Option<Vec<String>>,
    pub relevant_local_storage: Option<Vec<String>>,
    pub timestamp: i64,
    pub last_updated: i64,
    pub tab_id: Option<i32>,
    pub tab_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDomainData {
    pub clinic: Option<ExtensionTokenData>,
    pub grafana: Option<ExtensionTokenData>,
}

// API 请求结构
#[derive(Debug, Deserialize)]
pub struct TokenUpdateRequest {
    pub domain: String,
    pub service: String,
    pub data: ExtensionTokenData,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct AuthDataRequest {
    pub timestamp: i64,
    pub source: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DataSyncRequest {
    pub tokens: HashMap<String, ExtensionDomainData>,
    pub timestamp: i64,
    pub sync_type: String,
}

// API 响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub api_version: String,
    pub supported_services: Vec<String>,
    pub update_interval: u64,
    pub features: HashMap<String, bool>,
}

// 应用状态 - 存储扩展数据
#[derive(Debug, Clone)]
pub struct ExtensionAppState {
    pub tokens: Arc<Mutex<HashMap<String, ExtensionDomainData>>>,
    pub last_update: Arc<Mutex<Instant>>,
    pub connection_status: Arc<Mutex<String>>,
}

impl ExtensionAppState {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
            last_update: Arc::new(Mutex::new(Instant::now())),
            connection_status: Arc::new(Mutex::new("disconnected".to_string())),
        }
    }

    pub fn update_connection_status(&self, status: &str) {
        if let Ok(mut conn_status) = self.connection_status.lock() {
            *conn_status = status.to_string();
        }
        if let Ok(mut last_update) = self.last_update.lock() {
            *last_update = Instant::now();
        }
    }
}

// 扩展控制器
pub struct ExtensionController;

impl ExtensionController {
    // 处理 Token 更新
    pub async fn update_tokens(
        State(state): State<ExtensionAppState>,
        Json(request): Json<TokenUpdateRequest>,
    ) -> Result<ResponseJson<ApiResponse<()>>, StatusCode> {
        println!("[TiHC Extension API] 收到 Token 更新: {} / {}", request.domain, request.service);

        // 更新存储的数据
        if let Ok(mut tokens) = state.tokens.lock() {
            let domain_data = tokens.entry(request.domain.clone()).or_insert_with(|| ExtensionDomainData {
                clinic: None,
                grafana: None,
            });

            match request.service.as_str() {
                "clinic" => domain_data.clinic = Some(request.data),
                "grafana" => domain_data.grafana = Some(request.data),
                _ => {
                    return Ok(ResponseJson(ApiResponse {
                        success: false,
                        message: format!("不支持的服务类型: {}", request.service),
                        data: None,
                        timestamp: chrono::Utc::now().timestamp(),
                    }));
                }
            }
        }

        state.update_connection_status("connected");

        // 这里可以添加业务逻辑，比如：
        // 1. 存储到数据库
        // 2. 更新用户会话状态
        // 3. 触发其他服务
        Self::process_token_data(&request.domain, &request.service, &state).await;

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "Token 更新成功".to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 处理认证数据
    pub async fn handle_auth_data(
        State(state): State<ExtensionAppState>,
        Json(request): Json<AuthDataRequest>,
    ) -> Result<ResponseJson<ApiResponse<()>>, StatusCode> {
        println!("[TiHC Extension API] 收到认证数据: {:?}", request.source);

        state.update_connection_status("connected");

        // 处理认证数据的业务逻辑
        // 例如：自动登录、会话验证等

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "认证数据已接收".to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 处理数据同步
    pub async fn handle_data_sync(
        State(state): State<ExtensionAppState>,
        Json(request): Json<DataSyncRequest>,
    ) -> Result<ResponseJson<ApiResponse<()>>, StatusCode> {
        println!("[TiHC Extension API] 数据同步: {} 个域名", request.tokens.len());

        // 全量更新数据
        if let Ok(mut tokens) = state.tokens.lock() {
            *tokens = request.tokens;
        }

        state.update_connection_status("connected");

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "数据同步完成".to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 心跳检测
    pub async fn ping(
        State(state): State<ExtensionAppState>,
    ) -> Result<ResponseJson<ApiResponse<String>>, StatusCode> {
        state.update_connection_status("connected");

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "pong".to_string(),
            data: Some("healthy".to_string()),
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 获取配置
    pub async fn get_config() -> Result<ResponseJson<ApiResponse<ConfigResponse>>, StatusCode> {
        let mut features = HashMap::new();
        features.insert("auto_login".to_string(), true);
        features.insert("token_refresh".to_string(), true);
        features.insert("data_sync".to_string(), true);

        let config = ConfigResponse {
            api_version: "1.0".to_string(),
            supported_services: vec!["clinic".to_string(), "grafana".to_string()],
            update_interval: 10000,
            features,
        };

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "配置获取成功".to_string(),
            data: Some(config),
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 获取当前存储的所有数据 - 供前端查询
    pub async fn get_all_tokens(
        State(state): State<ExtensionAppState>,
    ) -> Result<ResponseJson<ApiResponse<HashMap<String, ExtensionDomainData>>>, StatusCode> {
        let tokens = if let Ok(tokens) = state.tokens.lock() {
            tokens.clone()
        } else {
            HashMap::new()
        };

        Ok(ResponseJson(ApiResponse {
            success: true,
            message: "数据获取成功".to_string(),
            data: Some(tokens),
            timestamp: chrono::Utc::now().timestamp(),
        }))
    }

    // 处理 Token 数据的业务逻辑
    async fn process_token_data(domain: &str, service: &str, state: &ExtensionAppState) {
        println!("[TiHC Extension API] 处理业务逻辑: {} / {}", domain, service);

        // 这里可以根据不同的服务类型执行不同的业务逻辑
        match service {
            "clinic" => {
                // Clinic 相关的业务逻辑
                // 例如：更新数据库连接配置、设置认证头等
                Self::process_clinic_tokens(state).await;
            }
            "grafana" => {
                // Grafana 相关的业务逻辑
                Self::process_grafana_tokens(state).await;
            }
            _ => {}
        }
    }

    async fn process_clinic_tokens(state: &ExtensionAppState) {
        // 处理 Clinic tokens 的具体业务逻辑
        // 例如：
        // 1. 更新 TiDB 连接配置
        // 2. 设置 API 请求的认证头
        // 3. 更新用户会话状态
        println!("[TiHC Extension API] 处理 Clinic Token 业务逻辑");
    }

    async fn process_grafana_tokens(state: &ExtensionAppState) {
        // 处理 Grafana tokens 的具体业务逻辑
        println!("[TiHC Extension API] 处理 Grafana Token 业务逻辑");
    }
}

// 创建扩展 API 路由
pub fn create_extension_routes() -> Router<ExtensionAppState> {
    Router::new()
        .route("/extension/tokens", post(ExtensionController::update_tokens))
        .route("/extension/auth-data", post(ExtensionController::handle_auth_data))
        .route("/extension/sync", post(ExtensionController::handle_data_sync))
        .route("/extension/ping", get(ExtensionController::ping))
        .route("/extension/config", get(ExtensionController::get_config))
        .route("/extension/data", get(ExtensionController::get_all_tokens))
        // .route("/extension/status", get(ExtensionController::get_status))
        // .route("/download/tihc-extension.crx", get(ExtensionController::download_extension))
}