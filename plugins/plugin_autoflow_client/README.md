# Plugin Autoflow Client

TiHC 系统的 Autoflow 客户端插件，负责将 tidb.ai（Autoflow）作为外部 AI 服务接入 TiHC 系统。

## 1. 插件概览

### 功能定位

- **AI 服务集成**：将 tidb.ai（Autoflow）作为外部 AI 服务接入 TiHC
- **会话管理**：负责会话创建、维护、销毁和上下文管理
- **消息转发**：接收前端请求，转发到 tidb.ai，并返回响应
- **状态管理**：保持无状态或轻量状态，可与其他插件（RCA、Prometheus、日志分析等）并行工作
- **流式支持**：支持 SSE/流式返回，提升前端 deep-chat 交互体验

### 技术特性

- **高性能**：基于 Rust + reqwest + tokio 异步实现
- **类型安全**：完整的 serde 类型定义和错误处理
- **可观测性**：内置日志、监控指标和链路追踪
- **容错性**：网络异常、超时、会话恢复等容错机制
- **可配置**：支持动态配置更新和多环境部署

## 2. 架构设计

### 2.1 整体架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │────│   TiHC Core     │────│  tidb.ai API   │
│   (Vue.js)      │    │   (Rust)        │    │  (External)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                       ┌─────────────────┐
                       │ Plugin Manager  │
                       └─────────────────┘
                              │
                       ┌─────────────────┐
                       │ Autoflow Client │
                       │    Plugin       │
                       └─────────────────┘
```

### 2.2 数据流

```
1. 用户输入 → 前端 RCA 问诊台
2. 前端 → TiHC API (/api/rca/chat)
3. TiHC Core → Plugin Manager → Autoflow Client Plugin
4. Plugin → tidb.ai API
5. tidb.ai Response → Plugin → TiHC Core → 前端
6. 前端显示 AI 回复
```

## 3. 功能特性

### 3.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 会话管理 | 创建、查询、关闭、重置会话，维护会话状态 | P0 |
| 消息转发 | 接收用户消息，转发到 tidb.ai，返回 AI 回复 | P0 |
| 流式输出 | 支持 SSE 流式响应，实时显示 AI 生成过程 | P0 |
| 错误处理 | 统一错误处理，网络异常、超时、API 错误处理 | P0 |
| 配置管理 | 支持动态配置 API URL、token、模型参数等 | P1 |
| 日志监控 | 请求日志、性能指标、调用统计 | P1 |
| 缓存优化 | 会话缓存、响应缓存，提升性能 | P2 |
| 限流控制 | API 调用频率限制，防止过载 | P2 |

### 3.2 扩展功能

- **多模型支持**：支持不同的 AI 模型选择
- **上下文管理**：智能上下文截断和优化
- **个性化配置**：用户级别的偏好设置
- **A/B 测试**：支持多个 AI 服务的对比测试

## 4. 技术实现

### 4.1 项目结构

```
plugin_autoflow_client/
├── Cargo.toml              # 依赖配置
├── README.md               # 项目文档
├── src/
│   ├── lib.rs              # 库入口，导出公共 API
│   ├── plugin.rs           # 插件主体，实现 Plugin trait
│   ├── client/
│   │   ├── mod.rs          # HTTP 客户端模块
│   │   ├── http.rs         # HTTP 请求封装
│   │   └── stream.rs       # SSE 流式处理
│   ├── session/
│   │   ├── mod.rs          # 会话管理模块
│   │   ├── manager.rs      # 会话管理器
│   │   └── store.rs        # 会话存储
│   ├── types/
│   │   ├── mod.rs          # 类型定义模块
│   │   ├── request.rs      # 请求类型
│   │   ├── response.rs     # 响应类型
│   │   └── config.rs       # 配置类型
│   ├── error/
│   │   ├── mod.rs          # 错误处理模块
│   │   └── types.rs        # 错误类型定义
│   └── utils/
│       ├── mod.rs          # 工具函数模块
│       ├── logging.rs      # 日志工具
│       └── metrics.rs      # 监控指标
├── tests/
│   ├── integration/        # 集成测试
│   │   ├── client_test.rs
│   │   └── session_test.rs
│   └── unit/              # 单元测试
│       ├── http_test.rs
│       └── types_test.rs
├── examples/              # 使用示例
│   └── basic_usage.rs
└── docs/                  # 详细文档
    ├── api.md
    ├── configuration.md
    └── troubleshooting.md
```

### 4.2 核心模块设计

#### 4.2.1 HTTP 客户端 (client/http.rs)

```rust
pub struct AutoflowHttpClient {
    client: reqwest::Client,
    config: Arc<RwLock<ClientConfig>>,
    metrics: Arc<ClientMetrics>,
}

impl AutoflowHttpClient {
    pub async fn send_message(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<ChatResponse, AutoflowError>;
    
    pub async fn send_message_stream(
        &self,
        session_id: &str,
        message: &str,
    ) -> Result<impl Stream<Item = Result<String, AutoflowError>>, AutoflowError>;
    
    pub async fn create_session(&self) -> Result<String, AutoflowError>;
    pub async fn close_session(&self, session_id: &str) -> Result<(), AutoflowError>;
}
```

#### 4.2.2 会话管理 (session/manager.rs)

```rust
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    config: SessionConfig,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub context: Vec<Message>,
    pub metadata: HashMap<String, String>,
}

impl SessionManager {
    pub fn create_session(&self) -> Result<String, AutoflowError>;
    pub fn get_session(&self, session_id: &str) -> Result<Session, AutoflowError>;
    pub fn update_session(&self, session_id: &str, message: Message) -> Result<(), AutoflowError>;
    pub fn close_session(&self, session_id: &str) -> Result<(), AutoflowError>;
    pub fn cleanup_expired_sessions(&self) -> Result<usize, AutoflowError>;
}
```

#### 4.2.3 类型定义 (types/)

```rust
// request.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub session_id: String,
    pub message: String,
    pub engine: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

// response.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub id: String,
    pub text: String,
    pub finished: bool,
    pub usage: Option<TokenUsage>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// config.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoflowConfig {
    pub base_url: String,
    pub api_key: String,
    pub default_engine: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub session_ttl_minutes: u32,
    pub rate_limit: RateLimitConfig,
}
```

#### 4.2.4 错误处理 (error/types.rs)

```rust
#[derive(Debug, thiserror::Error)]
pub enum AutoflowError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
}

impl From<AutoflowError> for PluginError {
    fn from(err: AutoflowError) -> Self {
        PluginError::External(format!("Autoflow: {}", err))
    }
}
```

## 5. API 接口设计

### 5.1 Plugin Trait 实现

```rust
#[async_trait]
impl Plugin for AutoflowClientPlugin {
    fn name(&self) -> &str {
        "autoflow_client"
    }
    
    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    
    async fn initialize(&mut self, config: Value) -> Result<(), PluginError>;
    
    async fn handle_request(&self, request: PluginRequest) -> Result<PluginResponse, PluginError>;
    
    async fn health_check(&self) -> Result<HealthStatus, PluginError>;
    
    async fn shutdown(&mut self) -> Result<(), PluginError>;
}
```

### 5.2 HTTP API 端点

| 端点 | 方法 | 描述 | 请求体 | 响应体 |
|------|------|------|--------|--------|
| `/api/rca/chat` | POST | 发送聊天消息 | ChatRequest | ChatResponse |
| `/api/rca/chat/stream` | POST | 流式聊天 | ChatRequest | SSE Stream |
| `/api/rca/session` | POST | 创建会话 | - | SessionResponse |
| `/api/rca/session/{id}` | DELETE | 关闭会话 | - | - |
| `/api/rca/sessions` | GET | 获取会话列表 | - | SessionList |
| `/api/rca/health` | GET | 健康检查 | - | HealthStatus |

### 5.3 事件和钩子

```rust
pub enum AutoflowEvent {
    SessionCreated { session_id: String },
    SessionClosed { session_id: String },
    MessageSent { session_id: String, message: String },
    MessageReceived { session_id: String, response: String },
    Error { session_id: Option<String>, error: AutoflowError },
}

pub trait AutoflowEventHandler {
    async fn on_event(&self, event: AutoflowEvent);
}
```

## 6. 配置管理

### 6.1 配置文件示例

```toml
# config/autoflow.toml
[autoflow]
base_url = "https://api.tidb.ai"
api_key = "${TIDB_AI_API_KEY}"
default_engine = "gpt-4"
timeout_seconds = 30
max_retries = 3

[session]
ttl_minutes = 60
max_concurrent = 100
cleanup_interval_minutes = 10

[rate_limit]
requests_per_minute = 60
burst_size = 10

[logging]
level = "info"
format = "json"
include_request_body = false
include_response_body = false

[metrics]
enabled = true
endpoint = "/metrics"
include_histograms = true
```

### 6.2 环境变量

```bash
# 必需的环境变量
TIDB_AI_API_KEY=your_api_key_here
TIDB_AI_BASE_URL=https://api.tidb.ai

# 可选的环境变量
AUTOFLOW_TIMEOUT=30
AUTOFLOW_MAX_RETRIES=3
AUTOFLOW_SESSION_TTL=3600
AUTOFLOW_LOG_LEVEL=info
```

## 7. 监控和可观测性

### 7.1 监控指标

```rust
pub struct AutoflowMetrics {
    // 请求指标
    pub requests_total: Counter,
    pub requests_duration: Histogram,
    pub requests_errors: Counter,
    
    // 会话指标
    pub sessions_active: Gauge,
    pub sessions_created: Counter,
    pub sessions_closed: Counter,
    
    // 流量指标
    pub tokens_consumed: Counter,
    pub response_size_bytes: Histogram,
}
```

### 7.2 日志格式

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "plugin": "autoflow_client",
  "session_id": "sess_123",
  "event": "message_sent",
  "duration_ms": 1250,
  "tokens_used": 150,
  "success": true
}
```

### 7.3 健康检查

```rust
pub struct HealthStatus {
    pub status: ServiceStatus,
    pub checks: Vec<HealthCheck>,
    pub timestamp: DateTime<Utc>,
}

pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

pub struct HealthCheck {
    pub name: String,
    pub status: CheckStatus,
    pub message: Option<String>,
    pub duration_ms: u64,
}
```

## 8. 错误处理和容错

### 8.1 错误分类

- **网络错误**：连接超时、DNS 解析失败
- **API 错误**：认证失败、请求格式错误、配额超限
- **会话错误**：会话不存在、会话过期
- **配置错误**：配置文件格式错误、必需参数缺失

### 8.2 重试策略

```rust
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

// 指数退避重试
impl RetryStrategy for ExponentialBackoff {
    async fn execute<F, R>(&self, operation: F) -> Result<R, AutoflowError>
    where
        F: Fn() -> Future<Output = Result<R, AutoflowError>>;
}
```

### 8.3 断路器模式

```rust
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    config: CircuitBreakerConfig,
}

pub enum CircuitState {
    Closed,
    Open { opened_at: Instant },
    HalfOpen,
}
```

## 9. 测试策略

### 9.1 单元测试

```rust
#[tokio::test]
async fn test_send_message_success() {
    let client = create_test_client().await;
    let response = client.send_message("test_session", "Hello").await;
    assert!(response.is_ok());
    assert!(!response.unwrap().text.is_empty());
}

#[tokio::test]
async fn test_session_management() {
    let manager = SessionManager::new(default_config());
    let session_id = manager.create_session().unwrap();
    assert!(manager.get_session(&session_id).is_ok());
    manager.close_session(&session_id).unwrap();
    assert!(manager.get_session(&session_id).is_err());
}
```

### 9.2 集成测试

```rust
#[tokio::test]
async fn test_end_to_end_chat() {
    let plugin = AutoflowClientPlugin::new(test_config()).await.unwrap();
    
    // 创建会话
    let session_response = plugin.handle_create_session().await.unwrap();
    
    // 发送消息
    let chat_response = plugin.handle_chat_request(
        &session_response.session_id,
        "What is TiDB?"
    ).await.unwrap();
    
    assert!(chat_response.text.contains("TiDB"));
    assert!(chat_response.finished);
}
```

### 9.3 性能测试

```rust
#[tokio::test]
async fn test_concurrent_requests() {
    let plugin = Arc::new(AutoflowClientPlugin::new(test_config()).await.unwrap());
    let tasks: Vec<_> = (0..100).map(|i| {
        let plugin = plugin.clone();
        tokio::spawn(async move {
            plugin.handle_chat_request("test_session", &format!("Message {}", i)).await
        })
    }).collect();
    
    let results = futures::future::join_all(tasks).await;
    assert_eq!(results.len(), 100);
}
```

## 10. 部署和运维

### 10.1 依赖项

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"
prometheus = "0.13"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
```

### 10.2 Docker 部署

```dockerfile
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/plugin_autoflow_client /usr/local/bin/
CMD ["plugin_autoflow_client"]
```

### 10.3 Kubernetes 配置

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: tihc-autoflow-plugin
spec:
  replicas: 3
  selector:
    matchLabels:
      app: tihc-autoflow-plugin
  template:
    metadata:
      labels:
        app: tihc-autoflow-plugin
    spec:
      containers:
      - name: plugin
        image: tihc/autoflow-plugin:latest
        env:
        - name: TIDB_AI_API_KEY
          valueFrom:
            secretKeyRef:
              name: tidb-ai-secret
              key: api-key
        resources:
          limits:
            memory: "512Mi"
            cpu: "500m"
          requests:
            memory: "256Mi"
            cpu: "250m"
```

## 11. 性能优化

### 11.1 连接池优化

```rust
pub struct ConnectionPoolConfig {
    pub max_idle_per_host: usize,
    pub keep_alive_timeout: Duration,
    pub pool_max_idle_per_host: usize,
}
```

### 11.2 缓存策略

```rust
pub struct CacheConfig {
    pub session_cache_size: usize,
    pub response_cache_ttl: Duration,
    pub enable_response_caching: bool,
}
```

### 11.3 资源管理

- 内存使用监控和限制
- CPU 使用率控制
- 网络连接数管理
- 文件句柄管理

## 12. 安全考虑

### 12.1 认证和授权

- API Key 安全存储和轮换
- 请求签名验证
- 会话劫持防护
- 权限控制

### 12.2 数据安全

- 敏感信息脱敏
- 传输加密（HTTPS）
- 日志数据保护
- PII 数据处理

### 12.3 安全配置

```rust
pub struct SecurityConfig {
    pub enable_tls: bool,
    pub certificate_path: Option<String>,
    pub private_key_path: Option<String>,
    pub allowed_origins: Vec<String>,
    pub rate_limit_enabled: bool,
}
```

## 13. 故障排查

### 13.1 常见问题

| 问题 | 症状 | 解决方案 |
|------|------|----------|
| 连接超时 | 请求长时间无响应 | 检查网络连接，调整超时配置 |
| 认证失败 | 401 错误 | 验证 API Key 是否正确和有效 |
| 会话丢失 | 上下文断裂 | 检查会话存储和清理策略 |
| 内存泄漏 | 内存使用持续增长 | 检查会话清理和对象释放 |

### 13.2 调试工具

```bash
# 查看插件日志
tail -f /var/log/tihc/autoflow_plugin.log

# 检查指标
curl http://localhost:9090/metrics | grep autoflow

# 健康检查
curl http://localhost:8080/health
```

## 14. 版本发布

### 14.1 版本规划

- **v0.1.0**: MVP 版本，基础聊天功能
- **v0.2.0**: 流式支持和会话管理
- **v0.3.0**: 监控指标和错误处理优化
- **v1.0.0**: 生产就绪版本

### 14.2 发布流程

1. 代码审查和测试
2. 版本标签和发布说明
3. Docker 镜像构建和推送
4. 文档更新
5. 部署验证

## 15. 贡献指南

### 15.1 开发环境

```bash
# 克隆项目
git clone https://github.com/your-org/tihc
cd tihc/plugins/plugin_autoflow_client

# 安装依赖
cargo build

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 15.2 提交规范

- feat: 新功能
- fix: 修复 bug
- docs: 文档更新
- test: 测试相关
- refactor: 重构
- perf: 性能优化

---

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

