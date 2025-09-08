use std::sync::Arc;
use microkernel::platform::message_bus::{BusMessage, MessageHandler, GLOBAL_MESSAGE_BUS, HandlerMode};
use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;
use crate::Counter;
use rmcp::{ErrorData as McpError, model::*};

/// MCP消息总线桥接器
/// 
/// 职责：
/// 1. 注册到消息总线，监听 "tihc-mcp" 主题
/// 2. 接收来自统一API层的MCP方法调用请求
/// 3. 将请求转发给Counter实例处理
/// 4. 将Counter的响应转换为消息总线格式返回
pub struct McpBusHandler {
    counter: Arc<Counter>,
}

impl McpBusHandler {
    pub fn new(counter: Arc<Counter>) -> Self {
        Self { counter }
    }

    /// 注册到消息总线
    pub fn register_to_bus(self: Arc<Self>) {
        // 注册处理 "tihc-mcp" 主题的请求 - 与统一API层保持一致
        let bus = Arc::clone(&*GLOBAL_MESSAGE_BUS);
        bus.register_chain("tihc-mcp", self, Some(HandlerMode::Request));
        
        tracing::info!("McpBusHandler registered to message bus with topic: tihc-mcp");
    }

    /// 处理MCP方法调用
    async fn handle_mcp_method(&self, method: &str, params: Value) -> Result<Value> {
        match method {
            "list_tools" => {
                // 当前采用手动构建工具列表的方式，原因如下：
                // 1. Counter 使用了 #[tool] 宏，生成的 tool_router 方法访问受限
                // 2. rmcp v0.6 的 ToolRouter::list() 方法不存在或私有
                // 3. 调用 rmcp 的 ServerHandler::list_tools() 需要复杂的 RequestContext
                // 
                // 理想的实现应该是：
                // let result = self.counter.list_tools(None, context).await?;
                // 
                // 但由于 API 限制，我们手动维护工具列表，确保与 Counter 的 #[tool] 宏保持同步
                let tools = vec![
                    // 简化的工具定义，与 Counter 中的 #[tool] 宏对应
                    serde_json::json!({
                        "name": "increment",
                        "description": "Increment the counter by 1"
                    }),
                    serde_json::json!({
                        "name": "decrement", 
                        "description": "Decrement the counter by 1"
                    }),
                    serde_json::json!({
                        "name": "get_value",
                        "description": "Get the current counter value"
                    }),
                    serde_json::json!({
                        "name": "say_hello",
                        "description": "Say hello to the client"
                    }),
                    serde_json::json!({
                        "name": "echo",
                        "description": "Echo back the provided message"
                    }),
                    serde_json::json!({
                        "name": "sum",
                        "description": "Calculate sum of provided numbers"
                    }),
                ];
                
                let result = serde_json::json!({
                    "tools": tools,
                    "next_cursor": null
                });
                Ok(result)
            }
            "call_tool" => {
                let request = serde_json::from_value::<CallToolRequestParam>(params)?;
                let tool_name = &request.name;
                
                // 直接调用Counter的方法 - 这些方法通过 #[tool] 宏定义
                let result = match tool_name.as_ref() {
                    "increment" => self.counter.increment().await?,
                    "decrement" => self.counter.decrement().await?,
                    "get_value" => self.counter.get_value().await?,
                    "say_hello" => self.counter.say_hello()?,
                    "echo" => {
                        let args = request.arguments.unwrap_or_default();
                        self.counter.echo(rmcp::handler::server::wrapper::Parameters(args))?
                    }
                    "sum" => {
                        let args = request.arguments.unwrap_or_default();
                        let struct_req: crate::StructRequest = serde_json::from_value(serde_json::Value::Object(args))?;
                        self.counter.sum(rmcp::handler::server::wrapper::Parameters(struct_req))?
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Unknown tool: {}", tool_name));
                    }
                };
                
                Ok(serde_json::to_value(result)?)
            }
            "list_prompts" => {
                // 手动构建 prompts 列表，因为 rmcp 的 PromptRouter 没有 list_prompts 方法
                let prompts = vec![
                    Prompt::new("example_prompt", Some("This is an example prompt that takes one required argument, message"), None),
                    Prompt::new("counter_analysis", Some("Analyze the current counter value and suggest next steps"), None),
                ];
                let result = ListPromptsResult { 
                    prompts, 
                    next_cursor: None 
                };
                Ok(serde_json::to_value(result)?)
            }
            "get_prompt" => {
                let request = serde_json::from_value::<GetPromptRequestParam>(params)?;
                let prompt_name = &request.name;
                
                // 简化实现，直接处理常见的 prompt 类型
                let result = match prompt_name.as_str() {
                    "example_prompt" => {
                        let message = request.arguments
                            .as_ref()
                            .and_then(|args| args.get("message"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Hello".to_string());
                        
                        GetPromptResult {
                            description: Some("Example prompt".to_string()),
                            messages: vec![
                                PromptMessage {
                                    role: PromptMessageRole::User,
                                    content: PromptMessageContent::text(format!(
                                        "This is an example prompt with your message here: '{}'", message
                                    )),
                                }
                            ],
                        }
                    }
                    "counter_analysis" => {
                        let goal = request.arguments
                            .as_ref()
                            .and_then(|args| args.get("goal"))
                            .and_then(|v| v.as_i64())
                            .unwrap_or(10) as i32;
                        
                        let current_value = {
                            let counter = self.counter.counter.lock().await;
                            *counter
                        };
                        
                        let difference = goal - current_value;
                        
                        GetPromptResult {
                            description: Some(format!("Counter analysis for reaching {} from {}", goal, current_value)),
                            messages: vec![
                                PromptMessage::new_text(
                                    PromptMessageRole::Assistant,
                                    "I'll analyze the counter situation and suggest the best approach.",
                                ),
                                PromptMessage::new_text(
                                    PromptMessageRole::User,
                                    format!(
                                        "Current counter value: {}\nGoal value: {}\nDifference: {}\n\nPlease analyze the situation and suggest the best approach to reach the goal.",
                                        current_value, goal, difference
                                    ),
                                ),
                            ],
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Unknown prompt: {}", prompt_name));
                    }
                };
                
                Ok(serde_json::to_value(result)?)
            }
            "list_resources" => {
                // 工具组件也可以提供一些资源
                let resources = vec![
                    RawResource::new("str:////Users/to/some/path/".to_string(), "cwd".to_string()).no_annotation(),
                    RawResource::new("memo://insights".to_string(), "memo-name".to_string()).no_annotation(),
                ];
                let result = ListResourcesResult { resources, next_cursor: None };
                Ok(serde_json::to_value(result)?)
            }
            "read_resource" => {
                let request = serde_json::from_value::<ReadResourceRequestParam>(params)?;
                match request.uri.as_str() {
                    "str:////Users/to/some/path/" => {
                        let result = ReadResourceResult {
                            contents: vec![ResourceContents::text("/Users/to/some/path/", request.uri)],
                        };
                        Ok(serde_json::to_value(result)?)
                    }
                    "memo://insights" => {
                        let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                        let result = ReadResourceResult {
                            contents: vec![ResourceContents::text(memo, request.uri)],
                        };
                        Ok(serde_json::to_value(result)?)
                    }
                    _ => Err(anyhow::anyhow!("Resource not found: {}", request.uri))
                }
            }
            "initialize" => {
                // MCP 初始化方法
                Ok(serde_json::json!({
                    "protocolVersion": "2025-06-18",
                    "capabilities": {
                        "tools": {
                            "listChanged": true
                        },
                        "prompts": {
                            "listChanged": true
                        },
                        "resources": {
                            "subscribe": false,
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": "tihc-mcp-server",
                        "version": "0.1.0"
                    }
                }))
            }
            "register" => {
                // 注册方法 - 返回成功确认
                Ok(serde_json::json!({
                    "success": true,
                    "message": "Component registered successfully"
                }))
            }
            "health_check" => {
                // 简单的健康检查
                Ok(serde_json::json!({
                    "status": "healthy",
                    "component": "rmcp-tools",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported method: {}", method))
            }
        }
    }
}

#[async_trait]
impl MessageHandler for McpBusHandler {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage> {
        tracing::debug!("McpBusHandler received message on topic: {}", msg.topic);
        
        if !msg.is_ok() {
            return Ok(BusMessage::error(&msg.topic, "Received error message"));
        }
        
        // 解析请求数据
        let request_data = msg.data.data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing request data"))?;
        
        // 提取方法名
        let method = request_data.get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        // 提取参数
        let params = request_data.get("params")
            .cloned()
            .unwrap_or(Value::Null);
        
        tracing::debug!("Handling MCP method: {} with params: {:?}", method, params);
        
        // 处理MCP方法调用
        match self.handle_mcp_method(method, params).await {
            Ok(result) => Ok(BusMessage::ok(&msg.topic, result)),
            Err(e) => {
                tracing::error!("MCP method '{}' failed: {}", method, e);
                Ok(BusMessage::error(&msg.topic, format!("MCP method '{}' failed: {}", method, e)))
            }
        }
    }
}
