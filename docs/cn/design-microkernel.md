# Microkernel 设计文档

版本：v1.0  
日期：2025-11-18  
作者：Aric  
适用场景：面向生产、支持流式 AI(TiDB.ai)、高并发 SSE、插件热加载、沙箱化插件(WASM)、分布式伸缩与多租户的微内核平台。



## 规范与风格

**微内核设计原则**：
本项目 microkernel 层严格遵循微内核（Microkernel）架构思想，核心只实现最小必要的基础能力（如事件总线、插件管理、会话管理、配置、日志等），所有业务逻辑、扩展能力、领域服务均以插件或外部模块形式集成，核心与插件解耦，便于安全隔离、热插拔和分布式扩展。

**领域驱动设计（DDD）原则**：
所有业务代码、插件代码应遵循 DDD（Domain-Driven Design）原则，聚焦领域模型、限界上下文、领域服务、聚合根等，确保业务复杂性可控、可演进，便于团队协作和长期维护。

**代码风格要求**：
- 所有 Rust 代码（包括 microkernel、业务、插件等）必须全部使用英文命名、英文注释，不允许出现中文标识符、中文注释或中文文档字符串。
- 设计文档可用中英文，但代码仓库内所有源代码、API、结构体、函数、注释、文档字符串等均要求全英文，确保国际化和开源友好。

## 目标概述

打造生产级、企业可用的 tihc microkernel 平台，核心能力包括：

1. 安全高效地代理和流式处理 TiDB.ai（支持 stream: true）；
2. 支持高并发 SSE/WebSocket 与万级长连接（如聊天场景）；
3. 插件热插拔与沙箱隔离（WASM/动态库），支持第三方插件生态；
4. 内置高性能 EventBus（广播+RPC），本地与分布式可扩展；
5. 会话历史、摘要、审计与 trace-id 全链路可追溯；
6. 多租户安全隔离、配额与合规审计。


## 设计原则与非功能需求

- **解耦**：所有模块通过事件或 RPC 交互，无直接依赖，便于插件热插拔与分布式扩展。
- **可靠性**：关键路径（如历史持久化、审计）保证至少一次写入，多副本/备份。
- **可观测性**：metrics/tracing/logs 全链路覆盖，trace_id 贯穿事件、RPC、日志。
- **安全性**：插件沙箱（WASM）、最小权限、密钥托管（Vault），RBAC、配额、审计全覆盖。
- **可扩展性**：支持水平扩缩容、分布式事件总线、持久化与重放。
- **低延迟流式响应**：TiDB.ai chunk 快速到前端并并行写历史。


## 总体架构

**EventBus 总线结构**：
- 本地总线采用 `tokio::broadcast` 实现高性能事件广播，`tokio::mpsc` + `oneshot` 实现点对点 RPC。所有事件封装为 `EventEnvelope`，带 trace_id、session_id、类型、时间戳等元数据，便于追踪与审计。
- 可选接入 Kafka/NATS/Redis Stream 作为 durable layer，实现全局事件复制与持久化，支持事件重放、离线分析、跨节点通知。
- 广播事件保证至少一次投递，RPC 支持超时与动态路由，handler 可动态注册。
- 总线接口抽象，便于未来切换底层实现或 mesh/分布式拓扑。

**插件系统**：
- 插件分为受信任动态库（Rust）和沙箱 WASM（Wasmtime/Wasmer），第三方插件运行于安全隔离环境，仅通过 Host API 交互。
- Host API 提供事件发布、RPC 调用、配置获取、日志、受限 KV 存储等接口，权限受控。
- 插件生命周期包括加载（签名校验）、初始化（注册 hooks/services）、事件驱动运行、资源安全卸载，支持热升级与安全隔离。
- 安全策略包括时间/内存配额、syscall 限制、IO 白名单、签名校验。

```
┌─────────────────────────────────────────────────────────────────┐
│                               Ingress                          │
│  (LoadBalancer / API Gateway - TLS terminating / Auth)         │
└───────────┬───────────────────────────────┬─────────────────────┘
            │                               │
            ▼                               ▼
  ┌───────────────────┐             ┌───────────────────┐
  │  Axum Edge Layer  │             │   Admin / Jobs    │
  │  (HTTP / SSE / WS │             │  (Cron, Workers)  │
  │   / gRPC / Auth ) │             └───────────────────┘
  └─────────┬─────────┘
            │ AppState (EventBus, Registry, Pools, Clients)
            ▼
  ┌─────────────────────────────────────────────────────────────────┐
  │                    Microkernel Core (per Pod)                  │
  │ ┌─────────────┐  ┌─────────────┐   ┌─────────────┐  ┌─────────┐ │
  │ │ AiProxy     │  │ SessionMgr  │   │ PluginMgr   │  │ Cache   │ │
  │ │ (tidb.ai)   │  │ (pipelines) │   │ (wasmtime)  │  │(moka/rs)│ │
  │ └────┬────────┘  └────┬────────┘   └────┬────────┘  └─────────┘ │
  │      │                │                 │                      │
  │      │                │                 │                      │
  │      └──────┬─────────┴─────────────────┴────────────┬─────────┘
  │             ▼                  EventBus (local + mesh) ▼
  │      ┌──────────────────────────────────────────────────────┐
  │      │  Broadcast channel  (tokio::broadcast / local ring)  │
  │      │  RPC channel (mpsc + oneshot map)                   │
  │      │  Durable layer (optional: Kafka / NATS / Redis Stream)│
  │      └──────────────────────────────────────────────────────┘
  └──────────────────────────────────────────────────────────────┘
            │                 │                    ▲
            ▼                 ▼                    │
  ┌────────────────┐  ┌─────────────────┐  ┌────────────────────┐
  │  Postgres /    │  │  Prometheus     │  │  Vault / Secrets   │
  │  Timescale     │  │  Grafana / OTel │  │  Manager (KMS)     │
  └────────────────┘  └─────────────────┘  └────────────────────┘
```


## 主要子系统详解

### 1. Axum 边缘层（HTTP / SSE / WebSocket / gRPC / MCP）
负责对外 API（如 /api/chat/stream）、认证、速率限制，将请求转为内部事件或 RPC。SSE 连接基于 session_id 订阅事件，handler 用 tokio::mpsc 连接内部总线。

### 2. Microkernel Core
统一管理 ServiceRegistry、EventBus、PluginManager、SessionManager，提供 start_session、call_service、publish_event 等 API。AppState 共享核心资源，Session 管理器维护会话上下文。

### 3. EventBus（广播+RPC+持久化）
本地用 tokio::broadcast 低延迟广播，tokio::mpsc+oneshot 实现 RPC。可选 Kafka/NATS/Redis Stream 做 durable layer，支持事件重放与分布式一致性。事件封装为 EventEnvelope，带 trace_id、session_id、类型、时间戳等。

### 4. Session Pipeline（流式处理）
每次聊天创建内存 pipeline，阶段可插拔（插件/模块）：入口、预处理、缓存、AI 调用、chunk 处理、SSE/WS 推送、历史写入、后处理。事件驱动，支持 backpressure 和 worker pool。

### 5. 插件系统（WASM 沙箱+动态加载）
插件分 Rust 动态库（受信任）和 WASM（第三方沙箱）。插件通过 Host API（事件、RPC、配置、日志、KV）与内核交互。生命周期包括加载、初始化、注册、事件驱动、卸载。沙箱策略：资源配额、syscall/IO 限制、签名校验、热升级。

### 6. ServiceRegistry / 依赖图
注册服务能力，支持能力发现、版本协商、依赖管理与健康检查，支持多租户隔离。

### 7. 持久化（DB/归档）
分层存储：Postgres/TimescaleDB（主存）、Elastic/Opensearch（全文检索）、S3（归档）、Kafka/NATS/Redis Stream（事件溯源）。支持 chunk 合并批写，提升写入效率。

### 8. 缓存层（本地+Redis）
本地 moka 高性能缓存，Redis 跨节点共享。优先本地读，miss fallback redis，必要时事件驱动失效。

### 9. 可观测性/安全
全链路 tracing（OpenTelemetry）、metrics（Prometheus）、结构化日志、审计日志、密钥托管（Vault/KMS）、认证（JWT/mTLS）、RBAC、配额。



## 