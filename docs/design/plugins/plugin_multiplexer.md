# plugin_multiplexer 设计文档

## 1. 概述

`plugin_multiplexer` 是一个单端口协议多路复用插件，作为系统唯一的网络入口，统一监听并分发所有通过 TCP 端口进入的连接。它能够自动判别协议类型（HTTP、RESTful、SSE、streamable-http-server、自定义二进制协议等），并将连接转交给对应的 handler（如 axum HTTP 服务、rmcp 的多种 transport handler）。本插件强调高扩展性、协议解耦和插件化架构。

---

## 2. 设计目标

- **单端口统一入口**：只用一个 TCP 端口，所有外部流量通过该端口进入。
- **协议自动判别**：根据连接首包内容或特征自动识别协议类型。
- **灵活分发**：不同协议类型的连接分发给不同 handler 处理，支持 HTTP 生态（axum/tower）、rmcp 多 transport、自定义协议等。
- **高可扩展性**：新协议 handler 可随时注册、扩展、替换，无需修改主流程。
- **本地通信支持**：支持 stdio、pipe、child-process 等本地通信，但这些不走网络端口。

---

## 3. 架构与核心模块

### 3.1 架构图

```
┌────────────────────────────┐
│  plugin_multiplexer (唯一端口)   │
│ ┌─────────────┬──────────┐ │
│ │ 协议判别器   │ Handler表 │ │
│ └─────┬───────┴─────┬────┘ │
│       │             │      │
│  [axum HTTP Service]│      │
│  [rmcp Transport Handler]  │
│  [自定义协议 Handler]      │
└───────┴──────────────┴─────┘
```

### 3.2 核心模块

- **Multiplexer 主循环**：唯一监听端口，接收连接，调度协议判别与分发。
- **协议判别器**：快速读取首包内容，判断协议类型（HTTP、streamable-http、SSE、自定义等）。
- **Handler 注册表**：注册并管理所有 handler（如 axum HTTP service、rmcp 各种 transport handler）。
- **本地通信适配**：本地 stdio/pipe/child-process handler 由插件统一生命周期管理，不占用端口。

---

## 4. 主要类型与接口

```rust
// 协议类型标识
enum ProtocolKind {
    Http,                  // 普通 HTTP/RESTful/SSE/streamable-http-server
    RmcpStreamableHttp,    // rmcp 的 streamable-http
    RmcpSse,               // rmcp 的 SSE
    CustomBinary,          // 其他自定义二进制协议
    // ...
}

// handler trait
#[async_trait::async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn handle(&self, stream: TcpStream) -> anyhow::Result<()>;
}

// multiplexer 主体
pub struct Multiplexer {
    handlers: HashMap<ProtocolKind, Arc<dyn ProtocolHandler>>,
    // 其它配置如 listen_addr
}
```

---

## 5. 工作流程

1. **监听唯一端口**：Multiplexer 通过 TcpListener 监听指定端口（如 0.0.0.0:8080）。
2. **判别协议类型**：对每个新连接 peek 首包，调用协议判别器识别协议类型。
3. **分发连接**：
    - 若为 HTTP/RESTful/SSE/streamable-http-server，则用 hyper::server::conn::Http::new().serve_connection(stream, axum_service) 交给 axum tower service。
    - 若为 rmcp 的其它 transport，则交给注册的 rmcp handler。
    - 若为自定义协议，则交给自定义 handler。
4. **本地通信**：
    - stdio/pipe/child-process handler 在插件初始化时自动注册和管理，不通过网络端口分发。

---

## 6. HTTP 与 streamable-http-server 的关系

- HTTP/RESTful、SSE、streamable-http-server 本质都是 HTTP 协议，均交由 axum handler 统一处理。
- 在 axum 路由内，可根据 path/header 再细分 streamable-http-server、SSE 等业务逻辑：
    - `/api/*` 为 RESTful
    - `/stream` 为 streamable-http-server
    - `/sse` 为 SSE

---

## 7. 支持的 rmcp transport 场景

| transport 名称            | 是否走端口 | multiplexer 处理方式                          |
|--------------------------|-----------|---------------------------------------------|
| streamable-http-server   | 是        | 判别为 HTTP，交 axum 路由 `/stream`         |
| transport-sse-server     | 是        | 判别为 HTTP，交 axum 路由 `/sse`            |
| transport-io (stdio)     | 否        | handler 在本地注册，不走端口                 |
| transport-child-process  | 否        | handler 在本地注册，不走端口                 |
| transport-sse-client     | 否        | handler 主动建立 SSE 连接，不走端口          |
| transport-streamable-http-client | 否 | handler 主动建立 HTTP 连接，不走端口         |

---

## 8. 代码结构建议

```
plugin_multiplexer/
  ├── lib.rs
  ├── protocol_detect.rs         // 协议判别器
  ├── handler_http.rs            // axum tower service handler
  ├── handler_rmcp.rs            // rmcp 各 transport 的 handler
  ├── handler_custom.rs          // 其它自定义协议 handler
  ├── handler_stdio.rs           // 本地 stdio/pipe handler
  ├── handler_childproc.rs       // 本地 child-process handler
  └── ...
```

---

## 9. 核心代码片段示意

```rust
let listener = TcpListener::bind("0.0.0.0:8080").await?;
loop {
    let (mut stream, _) = listener.accept().await?;
    tokio::spawn(async move {
        let mut buf = [0u8; 16];
        let n = stream.peek(&mut buf).await.unwrap();
        match detect_protocol(&buf[..n]) {
            ProtocolKind::Http => {
                hyper::server::conn::Http::new()
                    .serve_connection(stream, axum_service.clone())
                    .await
                    .unwrap();
            }
            ProtocolKind::RmcpStreamableHttp => {
                rmcp_streamable_handler.handle(stream).await.unwrap();
            }
            ProtocolKind::CustomBinary => {
                custom_handler.handle(stream).await.unwrap();
            }
        }
    });
}
```

---

## 10. 总结

- plugin_multiplexer 作为单端口多协议分发器，实现了高效、优雅的协议解耦与扩展。
- HTTP 相关协议归入统一 HTTP handler（axum/tower），业务细分在路由内部完成。
- rmcp 的多 transport、以及本地通信（stdio/pipe/child-process）均由 handler 注册与管理，插件架构清晰，利于维护和演进。
