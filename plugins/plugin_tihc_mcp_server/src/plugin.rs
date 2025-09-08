use std::sync::Arc;
use crate::{Counter, bus_handler::McpBusHandler};

/// 自动注册 MCP handlers 到消息总线
#[ctor::ctor]
fn init_mcp_plugin() {
    // 创建Counter实例
    let counter = Arc::new(Counter::new());
    
    // 创建消息总线处理器
    let bus_handler = Arc::new(McpBusHandler::new(counter));
    
    // 注册到消息总线
    bus_handler.register_to_bus();
    
    tracing::info!("MCP plugin initialized and registered to message bus");
}
