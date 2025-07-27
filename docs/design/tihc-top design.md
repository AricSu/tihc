
This is the complete, structured **"TiDB Intelligent Health Check (tihc)" Design Document**, covering the integration of microkernel architecture and DDD, with a focus on plugin communication, module boundaries, technology choices, and extension strategies.

---

# 📘 TiDB Intelligent Health Check (tihc) — Architecture Design Document

---

## 1️⃣ Project Goals

`tihc` is a CLI + Web integrated tool platform for DBAs, aiming to provide:

* **TiDB cluster inspection and diagnostics**
* **Slow log and performance analysis**
* **DDL change checking**
* **GitHub bug analysis and alerting**
* **Future root cause analysis (RCA/AWR-like features)**

Supports plugin-based extension, solid domain modeling, cross-platform deployment, and self-contained packaging.

---

## 2️⃣ Core Architectural Principles

| Layer         | Pattern/Approach              | Description                                 |
| ------------- | ---------------------------- | ------------------------------------------- |
| Core Platform | Microkernel Architecture      | Plugin scheduling/lifecycle/interface mgmt   |
| Plugin Design | DDD + Clean Architecture     | Each plugin is a bounded context, single responsibility |
| Plugin Comm   | Service Registry + Event/Command Bus | Decoupled plugin invocation                |
| Startup Mode  | CLI + Web Server             | Single binary, self-contained deployment    |

---

## 3️⃣ Overall Architecture Diagram (Logical View)

```
+-----------------------------------------------------+
|                 CLI / Web Server Entry Point        |
+-----------------------------------------------------+
|              🌐 Microkernel Core                    |
| +-----------------------------------------------+ |
| | Core Services                                 | |
| | - ConfigService                               | |
| | - LoggingService (tracing)                    | |
| | - DatabaseService (SQLx)                      | |
| | - MetricsService (Prometheus)                 | |
| | - EventBus / CommandBus                       | |
| | - ServiceRegistry (Plugin Service Registry)   | |
| +-----------------------------------------------+ |
| | Plugin Management (PluginManager)             | |
| | - Plugin discovery/loading/lifecycle mgmt     | |
| | - Plugin hot-reload (future)                  | |
| +-----------------------------------------------+ |
+-----------------------------------------------------+
|                📦 Plugin System (DDD Context)      |
| Plugin = Bounded Context, each plugin encapsulates its own domain and services |
| +-------------------------------------------------+ |
| | LossyDDLChecker       | Diagnose lossy DDL risks  | |
| | SlowLogParser         | Parse slow.log and import | |
| | GitHubIssueTracker    | GitHub issue mapping      | |
| | RCAEngine             | Root cause analysis (AWR/ADDM) | |
| | SQL Editor            | Visual SQL editor         | |
| | ProfileCollector      | Profile & metrics capture | |
| | AlertWebhook          | Alert push & config       | |
| +-------------------------------------------------+ |
+-----------------------------------------------------+
|              🧠 DDD Layer Structure in Each Plugin  |
| +-----------------------------------------------+ |
| | domain         | Domain model/rules/entities/events| |
| | application    | Use case layer/domain service coordination | |
| | infrastructure | DB/HTTP/Prometheus implementation| |
| | interface      | CLI/Web API layer                | |
| +-----------------------------------------------+ |
+-----------------------------------------------------+
|             📡 External Dependencies/Data Sources (Unified Adapter) |
| +-------------------------------------------------+ |
| | SQLx + TiDB / MySQL / PG                        | |
| | DuckDB embedded analytics DB                    | |
| | Prometheus / Grafana HTTP API                   | |
| | profile API capture (tidb/tikv/pd/ticdc)        | |
| +-------------------------------------------------+ |
```

---

## 4️⃣ Plugin Communication Mechanism

### ✅ Inter-plugin Calls: ServiceRegistry + Dependency Inversion Principle

**Core Idea**:

1. Plugin A defines and implements a `trait` interface (e.g., `DdlCheckerService`).
2. Plugin A registers the interface with the core `ServiceRegistry` during registration.
3. Plugin B obtains the capability via `registry.resolve::<dyn DdlCheckerService>()`.

Thus, **plugins are decoupled and communicate only via trait interfaces**; the core does not depend on concrete plugin implementations.

### 🔁 Plugin Event Propagation: EventBus + CommandBus

* Plugins do not need to be aware of each other; events are broadcast (e.g., DDL event triggers alert plugin).
* CommandBus can be used for CLI/Web to dispatch UseCase handlers in plugins.

---

## 5️⃣ Plugin Directory Structure (Example)

```text
plugin-lossy-ddl/
├── domain/
│   ├── rule.rs
│   └── model.rs
├── application/
│   └── lossy_ddl_service.rs
├── infrastructure/
│   └── parser_adapter.rs
├── interface/
│   └── cli.rs / web.rs
├── plugin.rs        // Plugin trait implementation + registration
├── lib.rs
```

### Plugin Registration Example

```rust
pub struct LossyDdlPlugin;

impl Plugin for LossyDdlPlugin {
    fn name(&self) -> &str { "lossy_ddl" }

    fn register(&mut self, ctx: &mut PluginContext) {
        ctx.register_command("check-lossy-ddl", LossyDdlHandler);
        ctx.service_registry.register::<dyn DdlCheckerService>(Arc::new(LossyDdlServiceImpl));
    }
}
```

---

## 6️⃣ Backend Key Technology Choices

| Module      | Technology                        | Reason                |
| ----------- | -------------------------------- | --------------------- |
| Web Framework | `axum` + `tower`               | High performance, composable |
| ORM         | `sqlx`                           | Zero runtime overhead, async |
| Local Analytics DB | `DuckDB`                   | Supports complex OLAP queries |
| Config Mgmt  | `config` + `serde`              | Multi-source config         |
| Logging      | `tracing`, `anyhow`, `thiserror`| Reliable diagnostics        |
| Metrics      | `prometheus-client`             | Internal observability      |
| Plugin Mgmt  | Custom PluginManager + trait    | Controllable plugin lifecycle |
| API Comm     | JSON REST API + `reqwest`       | Easy integration (e.g. Grafana) |

---

## 7️⃣ Frontend Architecture (Vue 3 + TS)

### 🧱 Tech Stack

| Technology  | Purpose      |
| ----------- | ------------|
| Vue 3       | UI framework |
| Vite        | Build tool   |
| TypeScript  | Static typing|
| Pinia       | State mgmt   |
| Axios       | HTTP client  |
| Naive UI    | High-quality UI components |
| ECharts     | Charting, diagnostics visualization |

### 📄 Page Modules

| Page         | Functionality  |
| ------------ | --------------|
| Dashboard    | Overview & status panel |
| Slow Log Analysis | Query/import/aggregation views |
| DDL Safety Check | Check SQL change risks |
| SQL Editor   | Execute/history mgmt |
| Profile Collection | Flamegraph display |
| Webhook Alert Config | Set push channels and rules |

---

## 8️⃣ CLI Command Design

```bash
# CLI mode diagnosis
tihc check lossy-ddl --file ddl.sql

# Start Web service + UI
tihc web --port 8080

# Plugin related
tihc plugin list
tihc plugin run slowlog-parser --file slow.log
```

---

## 9️⃣ Testing Strategy

| Layer          | Test Approach                        |
| -------------- | ------------------------------------|
| Domain         | Unit tests                           |
| Application    | Use case combination tests           |
| Interface      | HTTP/CLI interface tests             |
| Plugin Integration | Plugin load/invoke tests          |
| Core Platform  | PluginManager & ServiceRegistry tests|

---

## 🔒 10️⃣ Packaging & Deployment

* Build backend: `cargo build --release`
* Build frontend: `pnpm build`
* Static embedding: use `include_dir!` or `rust-embed`
* Single binary packaging: no external dependencies, supports container deployment

---

## 🛤️ 11️⃣ Roadmap (Milestones)

| Phase  | Features                                 |
| ------ | -----------------------------------------|
| MVP    | CLI mode, lossy ddl check, slow log parsing, Prometheus metrics |
| Alpha  | Web UI, GitHub Tracker, SQL Editor, Webhook |
| Beta   | Profile collection, Grafana integration, inspection report generation |
| GA     | RCA framework, rule/model-driven inference, plugin marketplace/hot-plug support |

---

## ✅ Architectural Design Principles Summary

* Plugins are DDD bounded contexts: strong consistency, high cohesion, low coupling.
* Microkernel only handles scheduling, registration, logging, config, not business logic.
* Plugin communication is unified via core interfaces (registry + trait).
* All modules are independently testable and support self-contained build/delivery.

---
## Directory Structure

```
tihc/                          # 根项目目录，Rust workspace
├── Cargo.toml                 # workspace 配置
├── cli/                      # CLI launcher
│   ├── Cargo.toml
│   └── src/
│       └── main.rs           # CLI parsing, core service dispatch
│
├── core/                     # Core lib: microkernel + DDD + plugin framework
│   ├── Cargo.toml
│   └── src/
│       ├── domain/           # Domain layer (entities, aggregates, events, rules)
│       ├── application/      # Application layer (use cases, domain service coordination)
│       ├── infrastructure/   # Infrastructure (DB, HTTP, external system adapters)
│       ├── interface/        # Interface layer (CLI adapters, Web API adapters, etc.)
│       ├── platform/         # Microkernel core (plugin mgmt, event bus, service registry)
│       └── plugin_api/       # Plugin public interface definitions (traits, etc.)
│
├── plugins/                  # Plugin collection, each as an independent crate (DDD context)
│   ├── plugin_lossy_ddl/     # LossyDDL diagnosis plugin
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugin_slowlog/       # Slow log parsing plugin
│   ├── plugin_github_issue/  # GitHub Issue tracking plugin
│   ├── plugin_rca_engine/    # Root cause analysis plugin
│   ├── plugin_sql_editor/    # SQL editor plugin
│   ├── plugin_profile_collector/ # Profile collection plugin
│   └── plugin_alert_webhook/ # Alert webhook plugin
│
├── backend/                      # Web service launcher, depends on core, provides REST API
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs           # Axum service entry point
│
├── frontend/       # Vue 3 frontend project, managed independently with npm
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── components/
│       ├── views/
│       ├── api/
│       ├── composables/
│       └── main.ts
│
├── common/                   # Common utility lib (types, helpers, error types, etc.)
│   ├── Cargo.toml
│   └── src/
│
├── scripts/                  # Scripts (build, release, DB migration, etc.)
│
└── docs/                     # Design docs, API specs, dev guidelines
```


## 文档注释规范
基础格式
使用三斜杠 /// 进行结构化注释。

注释必须为完整、规范的英文句子，首字母大写，结尾使用句号。

统一使用 Markdown 语法支持格式化（rustdoc 默认支持）。

📚 注释对象与规则
1. Modules（mod）
使用 //! 放在模块文件开头，描述模块职责、用途、暴露内容。

rust
复制
编辑
//! Handles the parsing and normalization of slow query logs.
//!
//! This module provides functions to load, analyze, and store
//! slow log entries for further inspection by diagnostic plugins.
2. Struct / Enum / Trait
✅ Struct / Enum
rust
复制
编辑
/// Represents a parsed slow log entry from TiDB or MySQL.
///
/// This structure is populated by the `SlowLogParser` plugin
/// and ingested into DuckDB for analysis.
pub struct SlowLogEntry {
    /// The SQL text of the slow query.
    pub sql: String,

    /// The total execution time in milliseconds.
    pub duration_ms: u64,
}
✅ Trait
rust
复制
编辑
/// Defines a diagnostic service for DDL safety checks.
///
/// Implementors are responsible for detecting risky or lossy
/// DDL patterns that may cause data loss or downtime.
pub trait DdlCheckerService {
    /// Analyzes the given SQL statements for lossy DDL patterns.
    fn check(&self, sql: &str) -> Result<Vec<CheckResult>>;
}
3. Function / Method
✅ 公共函数（包括 async/handler）
rust
复制
编辑
/// Runs the lossy DDL check on the specified SQL input.
///
/// Returns a list of detected issues or an empty list if the input is safe.
pub fn check_lossy_ddl(input: &str) -> anyhow::Result<Vec<CheckResult>> { ... }
⚠️ 私有函数（仅必要时）
rust
复制
编辑
// Parses an individual SQL statement into an AST node.
// Used internally by the lossy DDL checker.
fn parse_stmt(sql: &str) -> Option<SqlStmt> { ... }
4. Constants / Type Aliases
rust
复制
编辑
/// Default duration threshold (in ms) for slow query classification.
pub const DEFAULT_SLOW_QUERY_THRESHOLD: u64 = 300;
rust
复制
编辑
/// Alias for a list of formatted DDL warnings.
pub type DdlWarnings = Vec<CheckResult>;
5. Errors
使用 thiserror + 文档注释说明错误含义。

rust
复制
编辑
/// Errors that can occur while parsing a slow log file.
#[derive(thiserror::Error, Debug)]
pub enum SlowLogParseError {
    /// File could not be opened or read.
    #[error("failed to read log file")]
    Io(#[from] std::io::Error),

    /// Log entry could not be parsed.
    #[error("invalid slow log format")]
    InvalidFormat,
}
6. Tests
测试函数可简要说明测试目标。

rust
复制
编辑
#[test]
/// Ensures that `parse_stmt` correctly detects CREATE TABLE statements.
fn test_parse_create_table() {
    ...
}
🔁 注释风格建议（最佳实践）
项目	推荐做法
命名	使用清晰一致的英文名称，避免缩写
动词	函数/方法首句应以“Does/Parses/Returns...”等动词开头
段落结构	第一段简要描述用途，后续段落用 Markdown 标题/列表分层
示例	对复杂行为使用 # Examples 块举例说明

示例：
rust
复制
编辑
/// Resolves all registered services that implement the specified trait.
///
/// This function is typically used by plugins to access capabilities
/// provided by other plugins via the shared `ServiceRegistry`.
///
/// # Examples
/// ```
/// let svc = registry.resolve::<dyn DdlCheckerService>().unwrap();
/// ```
🚫 禁止事项
❌ 禁止在任何代码注释中使用中文

❌ 不要使用行内 // 中文说明

❌ 不要将设计性、逻辑性的描述藏在代码中，应移至设计文档（/docs）

📦 插件注释示例（完整）
rust
复制
编辑
/// A plugin that checks for lossy or unsafe DDL statements.
///
/// This plugin parses SQL files or CLI input and flags any DDL operations
/// that could result in data loss (e.g., `DROP COLUMN`, `MODIFY COLUMN` with shrink).
pub struct LossyDdlPlugin;

impl Plugin for LossyDdlPlugin {
    fn name(&self) -> &str { "lossy_ddl" }

    /// Registers the plugin with the provided runtime context.
    ///
    /// This includes command handlers, service trait implementations,
    /// and any event subscriptions if needed.
    fn register(&mut self, ctx: &mut PluginContext) {
        ctx.register_command("check-lossy-ddl", LossyDdlHandler);
        ctx.service_registry
            .register::<dyn DdlCheckerService>(Arc::new(LossyDdlServiceImpl));
    }
}
🧪 开发期间辅助注释规范
开发期间可使用临时 TODO / FIXME 注释，但必须是英文：

rust
复制
编辑
// TODO: Implement fallback when service not found.
// FIXME: This fails on malformed input; needs better validation.
开发完成后应清理多余注释，并保留必要的文档注释和维护性说明。

🗂️ 推荐工具链
工具	说明
rust-analyzer	提示文档结构、跳转与补全
cargo doc	编译 API 文档 (target/doc)
cargo clippy	提示注释格式错误与未使用文档