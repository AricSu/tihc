
# plugin_observability 设计文档（重构版）

## 1. 背景与目标

`plugin_observability` 是 Inspection / RCA Engine 的核心数据采集与管理模块，旨在为上层引擎提供统一、标准化的数据接口，屏蔽底层数据源差异，提升系统可观测性和可维护性。

**重构目标：**
- ✅ 统一 HTTP 数据源抽象，消除 Clinic 和 Prometheus 的重复逻辑
- ✅ 集成 Polars DataFrame 支持，提供强大的数据分析能力
- ✅ 提供工厂模式的数据源创建，支持动态扩展
- ✅ 保持向后兼容性，同时提供现代化的统一接口

**支持数据类型：**
- 数据库相关：日志、监控指标、系统表、系统参数、进程启动参数
- Kubernetes 相关：集群 API、子系统信息（Etcd、CNI、Ingress、Kibana 等）
- 数据分析：DataFrame 格式的结构化数据处理和时间序列分析

---

## 2. 重构后架构总览

```text
                   +---------------------------+
                   |   UnifiedDataManager      |
                   |  (基于 DataSource trait)   |
                   +-------------+-------------+
                                 |
          +----------------------+----------------------+
          |                      |                      |
          v                      v                      v
    +-------------+        +-------------+        +-------------+
    | Prometheus  |        |   Clinic    |        |  Other      |
    | DataSource  |        | DataSource  |        | DataSource  |
    +------+------+        +------+------+        +------+------+
           |                      |                      |
           v                      v                      v
    +------|------+        +------|------+        +------|------+
    | HttpDataSource (统一 HTTP 抽象)                       |
    | - 通用请求处理                                        |
    | - Header 管理                                        |
    | - 连接测试                                           |
    +----------------------------------------------------+

          +----------------------+
          |    DataProcessor     |
          | (Polars 数据分析)     |
          +----------+-----------+
                     |
       +-------------+-------------+
       |                           |
       v                           v
  DataFrame 转换           数据统计分析
  - prometheus_to_df      - basic_stats
  - clinic_cluster_to_df  - detect_anomalies
```

---

## 3. 核心模块设计

### 3.1 统一数据源抽象

**DataSource Trait：**
```rust
#[async_trait]
pub trait DataSource: Send + Sync {
    async fn test_connection(&self) -> Result<ConnectionTestResult, String>;
    async fn query(&self, query: &str, params: Option<HashMap<&str, String>>) -> Result<QueryResult, String>;
}
```

**统一查询结果：**
```rust
pub enum QueryResult {
    Metrics { data: serde_json::Value },
    Raw { raw_data: String },
    Json { json: serde_json::Value },
    DataFrame { df: DataFrame },           // 🆕 Polars 支持
    TimeSeries { df: DataFrame, timestamp_col: String, value_col: String },
}
```

### 3.2 HTTP 数据源统一抽象

**HttpDataSource：**
- 统一的 HTTP 客户端管理
- 通用的请求处理逻辑
- 标准化的 Header 和认证处理
- 健康检查通用方法

**优势：**
- 消除了 Clinic 和 Prometheus 之间 90% 的重复代码
- 统一的错误处理和日志记录
- 简化了新数据源的添加流程

### 3.3 数据处理工具

**DataProcessor：**
```rust
impl DataProcessor {
    // DataFrame 转换
    pub fn prometheus_to_dataframe(data: &serde_json::Value) -> PolarsResult<DataFrame>;
    pub fn clinic_cluster_to_dataframe(data: &serde_json::Value) -> PolarsResult<DataFrame>;
    
    // 数据分析
    pub fn basic_stats(df: &DataFrame, column: &str) -> Result<HashMap<String, f64>, String>;
    pub fn detect_anomalies_simple(df: &DataFrame, value_col: &str, threshold: f64) -> Result<Vec<bool>, String>;
}
```

### 3.4 工厂模式数据源创建

**create_data_source 工厂函数：**
```rust
pub fn create_data_source(config: DataSourceConfig) -> Box<dyn DataSource> {
    match config {
        DataSourceConfig::Prometheus { base_url, cookie } => {
            // 创建 Prometheus 数据源
        }
        DataSourceConfig::Clinic { base_url, apikey, cookie, csrf_token } => {
            // 创建 Clinic 数据源
        }
    }
}
```

### 3.5 统一数据管理器

**UnifiedDataManager：**
```rust
impl UnifiedDataManager {
    pub fn add_data_source(&mut self, name: String, config: DataSourceConfig);
    pub async fn query(&self, source_name: &str, query: &str, params: Option<HashMap<&str, String>>) -> Result<QueryResult, String>;
    pub async fn test_all_connections(&self) -> HashMap<String, ConnectionTestResult>;
}
```

---

## 4. 重构成果

### 4.1 代码简化
- **重复逻辑消除：** Clinic 和 Prometheus 共享 HTTP 处理逻辑
- **模块化设计：** 清晰的职责分离和接口设计
- **代码复用：** 统一的 HttpDataSource 可被任何 HTTP-based 数据源使用

### 4.2 功能增强
- **Polars 集成：** 强大的 DataFrame 数据处理能力
- **时间序列支持：** 专门的时间序列数据结构
- **数据分析工具：** 内置统计分析和异常检测

### 4.3 可扩展性
- **工厂模式：** 轻松添加新的数据源类型
- **trait 抽象：** 统一接口支持任何数据源实现
- **配置驱动：** 通过配置而非代码添加数据源

### 4.4 向后兼容
- **类型别名：** 保持旧的数据类型接口
- **DataManager trait：** 维持原有接口的兼容性
- **渐进式迁移：** 支持逐步迁移到新架构

---

## 5. 技术栈

- **核心框架：** Rust + async-trait
- **HTTP 客户端：** reqwest
- **数据处理：** Polars DataFrame
- **JSON 处理：** serde_json
- **日志记录：** tracing
- **异步运行时：** Tokio

---

## 6. 部署与使用

### 6.1 基本使用
```rust
let mut manager = UnifiedDataManager::new();

// 添加 Prometheus 数据源
manager.add_data_source("prometheus".to_string(), DataSourceConfig::Prometheus {
    base_url: "http://grafana:3000".to_string(),
    cookie: "session_token".to_string(),
});

// 添加 Clinic 数据源
manager.add_data_source("clinic".to_string(), DataSourceConfig::Clinic {
    base_url: "http://clinic-api:8080".to_string(),
    apikey: Some("api_key".to_string()),
    cookie: None,
    csrf_token: None,
});

// 查询数据
let result = manager.query("prometheus", "up", None).await?;
match result {
    QueryResult::DataFrame { df } => {
        // 使用 Polars DataFrame 进行数据分析
        let stats = DataProcessor::basic_stats(&df, "value")?;
    }
    _ => // 处理其他结果类型
}
```

### 6.2 数据分析示例
```rust
// Prometheus 指标转 DataFrame
let df = DataProcessor::prometheus_to_dataframe(&prometheus_result)?;

// 基础统计分析
let stats = DataProcessor::basic_stats(&df, "value")?;

// 异常检测
let anomalies = DataProcessor::detect_anomalies_simple(&df, "value", 2.0)?;
```

---

## 7. 性能与安全

- **性能优化：** Polars 提供高性能的数据处理，异步 HTTP 客户端支持并发请求
- **内存效率：** DataFrame 列式存储，减少内存占用
- **连接池：** reqwest 自动管理 HTTP 连接池
- **错误处理：** 统一的错误处理和重试机制
- **安全性：** 支持多种认证方式（Cookie、API Key、CSRF Token）

---

## 8. 未来规划

- **高级数据分析：** 更多 Polars 分析功能，如窗口函数、聚合、联接
- **流式处理：** 支持实时数据流处理
- **缓存策略：** 智能缓存机制提升性能
- **监控指标：** 暴露插件自身的运行指标
- **配置管理：** 动态配置更新和热重载
- **数据源扩展：** 支持更多数据源类型（Redis、InfluxDB、云厂商 API 等）
