# TiHC 插件系统

TiHC（TiDB Health Check）插件系统是一个基于微内核架构的可扩展平台，提供数据库健康检查、性能监控、根因分析等核心功能。每个插件都是独立的功能模块，通过统一的插件接口与核心系统协作。

## 🏗️ 插件架构

```
TiHC Core (Microkernel)
├── Plugin Manager (插件管理器)
├── Message Bus (消息总线)
└── Plugin Registry (插件注册表)
     ├── Data Collection (数据采集类)
     ├── Analysis Engine (分析引擎类)
     ├── Integration (集成服务类)
     └── Utility (工具类)
```

## 📋 插件清单

### 🔍 数据采集类

#### [plugin_observability](./plugin_observability/) - 可观测性数据源
- **功能**: 统一的数据源管理和采集接口
- **支持**: Prometheus、Clinic、Kubernetes API 等多种数据源
- **特性**: DataFrame 数据处理、HTTP 抽象、连接池管理
- **状态**: ✅ 生产就绪

#### [plugin_slowlog](./plugin_slowlog/) - 慢日志分析
- **功能**: TiDB/MySQL 慢查询日志采集、解析和存储
- **支持**: 多种日志格式、实时监控、历史数据分析
- **特性**: 异步处理、批量导入、结构化存储
- **状态**: ✅ 生产就绪

#### [plugin_profile_collector](./plugin_profile_collector/) - 性能画像采集
- **功能**: 系统性能指标采集和画像生成
- **支持**: CPU、内存、IO、网络等系统指标
- **特性**: 多维度画像、趋势分析、基线对比
- **状态**: 🚧 开发中

### 🧠 分析引擎类

#### [plugin_causality_engine](./plugin_causality_engine/) - 根因分析引擎
- **功能**: 基于因果模型的智能根因分析
- **技术**: 结构因果模型（SCM）、贝叶斯推理、不确定性处理
- **特性**: 逆向推理、概率化输出、已知问题匹配
- **状态**: 🚧 开发中

#### [plugin_inspection_engine](./plugin_inspection_engine/) - 巡检引擎
- **功能**: 自动化数据库健康检查和巡检报告
- **支持**: 规则引擎、自定义检查项、报告生成
- **特性**: 定时巡检、智能告警、趋势分析
- **状态**: 🚧 开发中

#### [plugin_autoflow_client](./plugin_autoflow_client/) - AI 助手客户端
- **功能**: 集成 tidb.ai（Autoflow）AI 服务
- **支持**: 自然语言问诊、智能诊断、流式对话
- **特性**: 会话管理、上下文维护、SSE 流式输出
- **状态**: 🚧 开发中

### 🔧 工具类

#### [plugin_sql_editor](./plugin_sql_editor/) - SQL 编辑器
- **功能**: 在线 SQL 查询和编辑工具
- **支持**: 多数据库连接、语法高亮、结果导出
- **特性**: 连接池管理、查询历史、安全控制
- **状态**: ✅ 生产就绪

#### [plugin_lossy_ddl](./plugin_lossy_ddl/) - DDL 变更跟踪
- **功能**: 数据库结构变更检测和跟踪
- **支持**: MySQL、TiDB DDL 变更监控
- **特性**: Go 语言集成、实时监控、变更历史
- **状态**: 🚧 开发中

### 🔗 集成服务类

#### [plugin_github_issue](./plugin_github_issue/) - GitHub 集成
- **功能**: 自动创建和管理 GitHub Issue
- **支持**: 问题自动上报、状态同步、标签管理
- **特性**: REST API 集成、模板化创建
- **状态**: 🚧 开发中

#### [plugin_alert_webhook](./plugin_alert_webhook/) - 告警 Webhook
- **功能**: 告警消息的 Webhook 转发服务
- **支持**: 多种告警源、自定义 Webhook 目标
- **特性**: 消息格式化、重试机制、批量发送
- **状态**: 🚧 开发中

## 🚀 快速开始

### 环境要求

- **Rust**: 1.70+
- **操作系统**: Linux, macOS, Windows
- **依赖**: Docker（可选，用于容器化部署）

### 构建所有插件

```bash
# 克隆项目
git clone https://github.com/your-org/tihc
cd tihc

# 构建所有插件
cargo build --workspace

# 运行测试
cargo test --workspace

# 构建发布版本
cargo build --workspace --release
```

### 单独构建插件

```bash
# 构建特定插件
cargo build -p plugin_observability
cargo build -p plugin_slowlog
cargo build -p plugin_sql_editor

# 运行特定插件测试
cargo test -p plugin_observability
```

## 📐 插件开发

### 插件结构规范

每个插件都遵循统一的项目结构：

```
plugin_name/
├── Cargo.toml              # 依赖配置
├── README.md               # 插件文档
├── src/
│   ├── lib.rs              # 库入口
│   ├── plugin.rs           # Plugin trait 实现
│   ├── application/        # 应用层（业务逻辑）
│   ├── domain/            # 领域层（核心模型）
│   ├── infrastructure/    # 基础设施层（数据访问）
│   └── interface/         # 接口层（API 定义）
├── tests/                 # 测试代码
└── examples/             # 使用示例
```

### Plugin Trait

所有插件都必须实现 `Plugin` trait：

```rust
#[async_trait]
pub trait Plugin: Send + Sync {
    /// 插件名称
    fn name(&self) -> &str;
    
    /// 插件描述
    fn description(&self) -> &str;
    
    /// 插件版本
    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
    
    /// 初始化插件
    async fn initialize(&mut self, config: Value) -> Result<(), PluginError>;
    
    /// 处理请求
    async fn handle_request(&self, request: PluginRequest) -> Result<PluginResponse, PluginError>;
    
    /// 健康检查
    async fn health_check(&self) -> Result<HealthStatus, PluginError>;
    
    /// 插件关闭
    async fn shutdown(&mut self) -> Result<(), PluginError>;
}
```

### 创建新插件

1. **生成插件骨架**:
```bash
cd plugins
cargo new plugin_your_name --lib
```

2. **添加依赖** (`Cargo.toml`):
```toml
[dependencies]
microkernel = { path = "../microkernel" }
common = { path = "../common" }
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
tracing = "0.1"
```

3. **实现 Plugin trait** (`src/plugin.rs`):
```rust
use microkernel::plugin_api::traits::Plugin;
use async_trait::async_trait;

pub struct YourPlugin {
    // 插件状态
}

#[async_trait]
impl Plugin for YourPlugin {
    fn name(&self) -> &str {
        "your_plugin"
    }
    
    fn description(&self) -> &str {
        "Your plugin description"
    }
    
    // 实现其他必需方法...
}
```

4. **注册插件** (`src/lib.rs`):
```rust
pub mod plugin;

pub use plugin::YourPlugin;

// 插件工厂函数
pub fn create_plugin() -> Box<dyn Plugin> {
    Box::new(YourPlugin::new())
}
```

## 🔧 配置管理

### 全局配置

插件配置通过 `_config.toml` 文件统一管理：

```toml
[plugins]
enabled = ["observability", "slowlog", "sql_editor"]

[plugin_observability]
prometheus_url = "http://localhost:9090"
clinic_url = "http://localhost:8080"

[plugin_slowlog]
mysql_host = "localhost"
mysql_port = 3306
batch_size = 1000

[plugin_sql_editor]
max_connections = 10
query_timeout = "30s"
```

### 环境变量

敏感配置通过环境变量传递：

```bash
# 数据库连接
export MYSQL_PASSWORD=your_password
export TIDB_HOST=your_tidb_host

# API 密钥
export GITHUB_TOKEN=your_github_token
export CLINIC_API_KEY=your_clinic_key
```

## 📊 监控和可观测性

### 指标收集

每个插件都内置监控指标收集：

```rust
use prometheus::{Counter, Histogram, Gauge};

pub struct PluginMetrics {
    pub requests_total: Counter,
    pub request_duration: Histogram,
    pub active_connections: Gauge,
}
```

### 日志规范

使用结构化日志格式：

```rust
use tracing::{info, warn, error};

info!(
    plugin = "observability",
    action = "data_collection",
    duration_ms = 150,
    "Data collection completed"
);
```

### 健康检查

每个插件提供健康检查端点：

```bash
# 检查单个插件健康状态
curl http://localhost:8080/api/health/observability

# 检查所有插件健康状态
curl http://localhost:8080/api/health/all
```

## 🧪 测试策略

### 单元测试

```bash
# 运行所有插件的单元测试
cargo test --workspace

# 运行特定插件的测试
cargo test -p plugin_observability

# 运行测试并显示输出
cargo test -- --nocapture
```
