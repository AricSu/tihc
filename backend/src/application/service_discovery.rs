// 服务发现服务 - TiHC Server 主动联系浏览器扩展
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub last_seen: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub server_url: String,
    pub port: u16,
    pub version: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug)]
pub struct ServiceDiscoveryService {
    pub connected_extensions: Arc<Mutex<HashMap<String, ExtensionInfo>>>,
    pub server_info: ServerInfo,
}

impl ServiceDiscoveryService {
    pub fn new(server_url: String, port: u16) -> Self {
        Self {
            connected_extensions: Arc::new(Mutex::new(HashMap::new())),
            server_info: ServerInfo {
                server_url,
                port,
                version: "1.0.0".to_string(),
                capabilities: vec![
                    "extension_discovery".to_string(),
                    "token_processing".to_string(),
                    "data_sync".to_string(),
                ],
            },
        }
    }

    // 广播服务发现消息
    pub async fn broadcast_discovery(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[TiHC Service Discovery] 开始广播服务发现...");

        // 1. 通过浏览器页面注入脚本的方式通知扩展
        self.inject_discovery_script().await?;

        // 2. 通过本地 UDP 广播（可选）
        // self.udp_broadcast().await?;

        // 3. 通过本地文件系统通信（可选）
        // self.file_system_discovery().await?;

        Ok(())
    }

    // 注入发现脚本到浏览器页面
    async fn inject_discovery_script(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这个方法需要在页面加载时注入脚本
        // 实际实现可能需要配合静态文件服务
        println!("[TiHC Service Discovery] 准备页面脚本注入...");
        
        // 创建发现脚本内容
        let discovery_script = self.create_discovery_script();
        
        // 这里可以将脚本保存到静态文件目录，供前端页面加载
        self.save_discovery_script(discovery_script).await?;
        
        Ok(())
    }

    // 创建发现脚本
    fn create_discovery_script(&self) -> String {
        format!(r#"
// TiHC 服务发现脚本
(function() {{
    console.log('[TiHC Service Discovery] 正在通知浏览器扩展...');
    
    // 向所有可能的扩展发送握手消息
    window.postMessage({{
        type: 'tihc_server_handshake',
        serverUrl: '{}',
        port: {},
        version: '{}',
        capabilities: {},
        timestamp: Date.now()
    }}, '*');
    
    // 监听扩展响应
    window.addEventListener('message', function(event) {{
        if (event.data.type === 'tihc_extension_handshake_response') {{
            console.log('[TiHC Service Discovery] 扩展响应:', event.data);
            
            // 通知服务器扩展已连接
            fetch('{}:{}/api/extension/register', {{
                method: 'POST',
                headers: {{
                    'Content-Type': 'application/json'
                }},
                body: JSON.stringify({{
                    extensionInfo: event.data.extensionInfo,
                    timestamp: Date.now()
                }})
            }}).then(response => {{
                if (response.ok) {{
                    console.log('[TiHC Service Discovery] 扩展注册成功');
                }} else {{
                    console.error('[TiHC Service Discovery] 扩展注册失败');
                }}
            }}).catch(error => {{
                console.error('[TiHC Service Discovery] 扩展注册错误:', error);
            }});
        }}
    }});
}})();
        "#, 
            self.server_info.server_url,
            self.server_info.port,
            self.server_info.version,
            serde_json::to_string(&self.server_info.capabilities).unwrap_or_default(),
            self.server_info.server_url,
            self.server_info.port
        )
    }

    // 保存发现脚本
    async fn save_discovery_script(&self, script: String) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        
        // 保存到静态文件目录
        let script_path = "static/js/service-discovery.js";
        
        // 确保目录存在
        if let Some(parent) = std::path::Path::new(script_path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(script_path, script)?;
        println!("[TiHC Service Discovery] 发现脚本已保存到: {}", script_path);
        
        Ok(())
    }

    // 注册扩展
    pub async fn register_extension(&self, extension_id: String, extension_info: ExtensionInfo) {
        let mut extensions = self.connected_extensions.lock().await;
        extensions.insert(extension_id.clone(), extension_info.clone());
        
        println!("[TiHC Service Discovery] 扩展已注册: {} (版本: {})", 
                extension_id, extension_info.version);
    }

    // 获取已连接的扩展
    pub async fn get_connected_extensions(&self) -> HashMap<String, ExtensionInfo> {
        self.connected_extensions.lock().await.clone()
    }

    // 检查扩展连接状态
    pub async fn is_extension_connected(&self, extension_id: &str) -> bool {
        self.connected_extensions.lock().await.contains_key(extension_id)
    }

    // 清理过期的扩展连接
    pub async fn cleanup_stale_connections(&self, timeout_seconds: i64) {
        let mut extensions = self.connected_extensions.lock().await;
        let current_time = chrono::Utc::now().timestamp();
        
        extensions.retain(|id, info| {
            let is_active = current_time - info.last_seen < timeout_seconds;
            if !is_active {
                println!("[TiHC Service Discovery] 清理过期扩展连接: {}", id);
            }
            is_active
        });
    }

    // 启动定期清理任务
    pub fn start_cleanup_task(service: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                service.cleanup_stale_connections(300).await; // 5分钟超时
            }
        });
    }
}

// 服务发现控制器
pub struct ServiceDiscoveryController;

impl ServiceDiscoveryController {
    // 注册扩展端点
    pub async fn register_extension(
        axum::extract::State(service): axum::extract::State<Arc<ServiceDiscoveryService>>,
        axum::extract::Json(request): axum::extract::Json<serde_json::Value>,
    ) -> Result<axum::response::Json<serde_json::Value>, axum::http::StatusCode> {
        
        if let Some(extension_info) = request.get("extensionInfo") {
            let extension_id = extension_info
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            let ext_info = ExtensionInfo {
                id: extension_id.clone(),
                version: extension_info
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                capabilities: extension_info
                    .get("capabilities")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    })
                    .unwrap_or_default(),
                last_seen: chrono::Utc::now().timestamp(),
            };
            
            service.register_extension(extension_id, ext_info).await;
            
            Ok(axum::response::Json(serde_json::json!({
                "success": true,
                "message": "扩展注册成功"
            })))
        } else {
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
    }

    // 获取服务发现状态
    pub async fn get_discovery_status(
        axum::extract::State(service): axum::extract::State<Arc<ServiceDiscoveryService>>,
    ) -> axum::response::Json<serde_json::Value> {
        let extensions = service.get_connected_extensions().await;
        
        axum::response::Json(serde_json::json!({
            "server_info": service.server_info,
            "connected_extensions": extensions,
            "total_extensions": extensions.len()
        }))
    }
}

// 创建服务发现路由
pub fn create_service_discovery_routes() -> axum::Router<Arc<ServiceDiscoveryService>> {
    axum::Router::new()
        .route("/extension/register", axum::routing::post(ServiceDiscoveryController::register_extension))
        .route("/discovery/status", axum::routing::get(ServiceDiscoveryController::get_discovery_status))
}