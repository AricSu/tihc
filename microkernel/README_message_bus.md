# MessageBusImpl 用法文档

本模块实现了基于 topic 的异步消息总线，支持插件/模块解耦通信。核心特性：
- 统一消息格式（BusMessage/MessageData）
- handler 注册支持链式调用、函数式编程、回调
- 广播/请求两种分发模式

## 主要类型
- `BusMessage`：消息载体，包含 topic 和数据
- `MessageData`：消息内容，包含 ok/data/error
- `HandlerMode`：分发模式（Broadcast/Request）
- `MessageHandler` trait：异步处理器接口
- `MessageBusImpl`：总线实现，支持 handler 注册和消息分发

## 注册 handler
### 链式注册
```rust
bus.register_chain("topic1", handler, HandlerMode::Broadcast)
   .register_chain("topic2", handler2, HandlerMode::Request);
```

### 函数式注册（闭包）
```rust
bus.register_fn("topic_fn", HandlerMode::Broadcast, |msg| {
    Ok(BusMessage::ok(msg.topic, "fn handler response"))
});
```

### 回调注册
```rust
bus.register_with_callback(
    "topic_cb",
    HandlerMode::Request,
    |msg| Ok(BusMessage::ok(msg.topic, "callback handler response")),
    |topic| println!("Handler registered for topic: {}", topic)
);
```

## 发送消息
- 广播：`bus.send(BusMessage::ok("topic", data)).await?` 返回所有 handler 响应
- 请求：`bus.request(BusMessage::ok("topic", data)).await?` 返回单个响应

## 典型场景
- 插件/模块通过 topic 注册 handler，实现解耦
- CLI/web API/插件均可通过消息总线分发和处理命令
- 支持链式/闭包/回调注册，业务代码更优雅

## 注意事项
- handler 必须实现 `MessageHandler` trait
- 闭包注册时建议捕获必要上下文
- 需在异步环境（如 tokio）下调用 send/request

---
如需更详细示例或 API 说明，请参考 `examples/message_bus_usage.rs` 或联系维护者。
