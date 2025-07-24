以下是整理后的、完整结构化的 **《TiDB Intelligent Health Check (tihc)》设计文档**，涵盖微内核架构与 DDD 模式的结合方式，并着重突出插件通信机制、模块边界、技术选型与扩展策略。

---

# 📘 TiDB Intelligent Health Check (tihc) — 架构设计文档

---

## 1️⃣ 项目目标

`tihc` 是一个面向 DBA 的 CLI + Web 综合工具平台，旨在提供：

* **TiDB 集群巡检与诊断能力**
* **慢日志与性能分析**
* **DDL 变更检查**
* **GitHub Bug 分析与告警**
* **未来的根因分析（RCA/AWR 类功能）**

支持插件化扩展、良好的领域建模、跨平台部署、自包含打包。

---

## 2️⃣ 核心架构理念

| 架构层面 | 采用模式                     | 说明               |
| ---- | ------------------------ | ---------------- |
| 核心平台 | 微内核架构 Microkernel        | 插件调度/生命周期管理/接口注册 |
| 插件结构 | DDD + Clean Architecture | 插件即限界上下文，职责单一    |
| 插件通信 | 服务注册中心 + 事件/命令总线         | 解耦插件之间的调用        |
| 启动模式 | CLI + Web Server         | 单 Binary，自包含部署   |

---

## 3️⃣ 总体架构图（逻辑视图）

```
+-----------------------------------------------------+
|                 CLI / Web Server 启动入口           |
+-----------------------------------------------------+
|              🌐 微内核核心（Microkernel）            |
| +-----------------------------------------------+ |
| | 核心服务 Core Services                        | |
| | - ConfigService                               | |
| | - LoggingService (tracing)                    | |
| | - DatabaseService (SQLx)                      | |
| | - MetricsService (Prometheus)                 | |
| | - EventBus / CommandBus                       | |
| | - ServiceRegistry (插件服务注册/发现)         | |
| +-----------------------------------------------+ |
| | 插件管理 PluginManager                        | |
| | - 插件发现 / 加载 / 生命周期管理              | |
| | - 插件热更新（后续支持）                      | |
| +-----------------------------------------------+ |
+-----------------------------------------------------+
|                📦 插件系统 Plugins (DDD Context)   |
| 插件 = 限界上下文，每个插件自包含领域与服务          |
| +-------------------------------------------------+ |
| | LossyDDLChecker       | 诊断 DDL 丢失风险         | |
| | SlowLogParser         | 解析 slow.log 入库         | |
| | GitHubIssueTracker    | GitHub issue 映射分析     | |
| | RCAEngine             | 根因分析 (AWR/ADDM 类)   | |
| | SQL Editor            | 可视化 SQL 编辑器         | |
| | ProfileCollector      | profile & metrics 抓取     | |
| | AlertWebhook          | 告警推送 & 配置           | |
| +-------------------------------------------------+ |
+-----------------------------------------------------+
|              🧠 每个插件内部 DDD 层级结构           |
| +-----------------------------------------------+ |
| | domain         | 领域模型 / 规则 / 实体 / 事件     | |
| | application    | 用例层 / 领域服务协调             | |
| | infrastructure | 数据库 / HTTP / Prometheus 实现 | |
| | interface      | CLI/Web API 层                   | |
| +-----------------------------------------------+ |
+-----------------------------------------------------+
|             📡 外部依赖 / 数据源支持（统一适配）      |
| +-------------------------------------------------+ |
| | SQLx + TiDB / MySQL / PG                        | |
| | DuckDB 嵌入式分析数据库                         | |
| | Prometheus / Grafana HTTP API                   | |
| | profile 接口抓取 (tidb/tikv/pd/ticdc)           | |
| +-------------------------------------------------+ |
```

---

## 4️⃣ 插件通信机制设计

### ✅ 插件之间相互调用：ServiceRegistry + 依赖倒置原则

**核心思路**：

1. 插件 A 定义并实现 `trait` 接口（例如 `DdlCheckerService`）。
2. 插件 A 在注册阶段将接口注册到核心 `ServiceRegistry`。
3. 插件 B 通过 `registry.resolve::<dyn DdlCheckerService>()` 获取能力。

这样，**插件之间解耦，仅通过 trait 接口通信**，核心不依赖具体插件实现。

### 🔁 插件事件传播：EventBus + CommandBus

* 插件之间无需主动感知，通过发布事件实现广播式通信（例如：DDL 事件触发告警插件）
* CommandBus 可用于 CLI/Web 调用调度各插件的 UseCase handler

---

## 5️⃣ 插件目录结构规范（示例）

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
├── plugin.rs        // Plugin Trait 实现 + 注册
├── lib.rs
```

### 插件注册代码

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

## 6️⃣ 后端关键技术选型

| 模块      | 技术                               | 理由                  |
| ------- | -------------------------------- | ------------------- |
| Web 框架  | `axum` + `tower`                 | 高性能、组合灵活            |
| ORM     | `sqlx`                           | 零运行时开销、异步           |
| 本地分析 DB | `DuckDB`                         | 支持复杂 OLAP 查询        |
| 配置管理    | `config` + `serde`               | 多来源配置               |
| 日志追踪    | `tracing`, `anyhow`, `thiserror` | 稳定可靠的诊断工具链          |
| 监控指标    | `prometheus-client`              | 内部状态可观测             |
| 插件管理    | 自定义 PluginManager + trait        | 插件生命周期可控            |
| 接口通信    | JSON REST API + `reqwest`        | 易集成其他服务，如 Grafana 等 |

---

## 7️⃣ 前端架构设计（Vue 3 + TS）

### 🧱 技术栈

| 技术         | 用途         |
| ---------- | ---------- |
| Vue 3      | UI 框架      |
| Vite       | 构建工具       |
| TypeScript | 静态类型       |
| Pinia      | 状态管理       |
| Axios      | 请求库        |
| Naive UI   | 高质量 UI 组件库 |
| ECharts    | 图表组件，诊断可视化 |

### 📄 页面模块

| 页面           | 功能            |
| ------------ | ------------- |
| Dashboard    | 概览 & 状态面板     |
| 慢日志分析        | 查询、导入、聚合视图    |
| DDL 安全检测     | 检查 SQL 变更风险   |
| SQL 编辑器      | 执行 / 历史记录管理   |
| Profile 采集   | flamegraph 展示 |
| Webhook 告警设置 | 设置推送通道和规则     |

---

## 8️⃣ CLI 命令设计

```bash
# CLI 模式下诊断
tihc check lossy-ddl --file ddl.sql

# 启动 Web 服务 + UI
tihc web --port 8080

# 插件相关
tihc plugin list
tihc plugin run slowlog-parser --file slow.log
```

---

## 9️⃣ 测试策略

| 层级            | 测试方式                               |
| ------------- | ---------------------------------- |
| domain 层      | 单元测试                               |
| application 层 | 用例组合测试                             |
| interface 层   | HTTP/CLI 接口测试                      |
| 插件集成          | 插件加载/调用测试                          |
| 核心平台          | PluginManager & ServiceRegistry 测试 |

---

## 🔒 10️⃣ 打包与部署

* 构建后端：`cargo build --release`
* 构建前端：`pnpm build`
* 静态嵌入：使用 `include_dir!` 或 `rust-embed`
* 单 Binary 打包：无需依赖外部服务，支持容器部署

---

## 🛤️ 11️⃣ Roadmap（阶段目标）

| 阶段    | 功能点                                      |
| ----- | ---------------------------------------- |
| MVP   | CLI 模式、lossy ddl 检测、慢日志解析、Prometheus 指标  |
| Alpha | Web UI、GitHub Tracker、SQL Editor、Webhook |
| Beta  | Profile 抓取、Grafana 数据集成、巡检报告生成           |
| GA    | RCA 框架、规则/模型驱动推理、插件市场/热插拔支持              |

---

## ✅ 架构设计原则总结

* 插件即 DDD 限界上下文：封闭一致性、高内聚低耦合
* 微内核只负责调度、注册、日志、配置等横向能力，不承载业务
* 插件通信统一通过核心接口（注册中心 + trait）
* 所有模块可独立测试，支持自包含构建与交付

---
## 目录结构

```
tihc/                          # 根项目目录，Rust workspace
├── Cargo.toml                 # workspace 配置
├── cli/                      # CLI 启动器
│   ├── Cargo.toml
│   └── src/
│       └── main.rs           # 命令行解析，调度核心服务
│
├── core/                     # 核心库：微内核架构 + DDD 分层 + 插件框架
│   ├── Cargo.toml
│   └── src/
│       ├── domain/           # 领域层（实体、聚合、领域事件、规则）
│       ├── application/      # 应用层（UseCase、领域服务协调）
│       ├── infrastructure/  # 基础设施层（数据库、HTTP、外部系统接口）
│       ├── interface/        # 适配层（CLI适配器、Web API适配器等）
│       ├── platform/         # 微内核平台核心（插件管理、事件总线、ServiceRegistry）
│       └── plugin_api/       # 插件公共接口定义（trait等）
│
├── plugins/                  # 插件合集，每个插件独立 crate（DDD 限界上下文）
│   ├── plugin_lossy_ddl/     # LossyDDL 诊断插件
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugin_slowlog/       # 慢日志解析插件
│   ├── plugin_github_issue/  # GitHub Issue 追踪插件
│   ├── plugin_rca_engine/    # 根因分析插件
│   ├── plugin_sql_editor/    # SQL 编辑器插件
│   ├── plugin_profile_collector/ # Profile 采集插件
│   └── plugin_alert_webhook/ # 告警推送插件
│
├── backend/                      # Web 服务启动器，依赖 core，提供 REST API
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs           # Axum 服务入口
│
├── frontend/       # Vue 3 前端项目，独立 npm 管理
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── components/
│       ├── views/
│       ├── api/
│       ├── composables/
│       └── main.ts
│
├── common/                   # 通用工具库（类型定义、辅助函数、错误类型等）
│   ├── Cargo.toml
│   └── src/
│
├── scripts/                  # 脚本（构建、发布、数据库迁移等）
│
└── docs/                     # 设计文档、API 说明、开发规范
```