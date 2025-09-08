use microkernel::platform::message_bus::{
    BusMessage, MessageHandler, MessageBus, HandlerMode, GLOBAL_MESSAGE_BUS
};
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use crate::Counter;
use rmcp::ServerHandler;
use rmcp::model::*;
use serde_json::{json, Value};

/// rmcp工具组件处理器 - 包装Counter，处理消息总线请求
/// 监听 tihc-mcp-tools topic，使用rmcp原生Counter的ServerHandler实现
pub struct RmcpToolsHandler {
    counter: Counter,
}

impl RmcpToolsHandler {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
    
    pub fn get_counter(&self) -> &Counter {
        &self.counter
    }

    /// 处理MCP方法调用，简化版本不使用完整的ServerHandler，直接调用工具方法
    async fn handle_mcp_method(&self, method: &str, params: Value) -> Result<Value> {        
        let result = match method {
            "health_check" => {
                Ok(json!({"status": "healthy", "component": "rmcp-tools"}))
            }
            "initialize" => {
                tracing::info!("rmcp工具组件初始化");
                let info = self.counter.get_info();
                Ok(serde_json::to_value(info)?)
            }
            "list_tools" => {
                // 手动构建工具列表，基于Counter的#[tool]宏定义
                let tools = vec![
                    Tool::new("increment", "Increment the counter by 1", serde_json::Map::new()),
                    Tool::new("decrement", "Decrement the counter by 1", serde_json::Map::new()),
                    Tool::new("get_value", "Get the current counter value", serde_json::Map::new()),
                    Tool::new("say_hello", "Say hello to the client", serde_json::Map::new()),
                    Tool::new("echo", "Repeat what you say", serde_json::Map::new()),
                    Tool::new("sum", "Calculate the sum of two numbers", serde_json::Map::new()),
                ];
                let result = ListToolsResult { tools, next_cursor: None };
                Ok(serde_json::to_value(result)?)
            }
            "call_tool" => {
                let request = serde_json::from_value::<CallToolRequestParam>(params)?;
                let tool_name = &request.name;
                
                // 根据工具名直接调用Counter的方法
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
                // 手动构建提示列表
                let prompts = vec![
                    Prompt::new("example_prompt", Some("This is an example prompt that takes one required argument, message"), None),
                    Prompt::new("counter_analysis", Some("Analyze the current counter value and suggest next steps"), None),
                ];
                let result = ListPromptsResult { prompts, next_cursor: None };
                Ok(serde_json::to_value(result)?)
            }
            "get_prompt" => {
                let request = serde_json::from_value::<GetPromptRequestParam>(params)?;
                let prompt_name = &request.name;
                
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
                                        "This is an example prompt with your message here: '{}'", &message
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
            "list_resource_templates" => {
                let result = ListResourceTemplatesResult {
                    next_cursor: None, 
                    resource_templates: Vec::new()
                };
                Ok(serde_json::to_value(result)?)
            }
            _ => {
                Err(anyhow::anyhow!("Unknown MCP method: {}", method))
            }
        };
        
        result
    }
}

#[async_trait]
impl MessageHandler for RmcpToolsHandler {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage> {
        // 处理来自 tihc-mcp-tools topic 的请求
        if msg.topic != "tihc-mcp-tools" {
            return Ok(BusMessage::error("invalid_topic", 
                format!("Expected 'tihc-mcp-tools', got '{}'", msg.topic)));
        }

        if let Ok(data) = msg.unwrap_data::<Value>() {
            if let (Some(method), Some(params)) = (
                data.get("method").and_then(|v| v.as_str()),
                data.get("params")
            ) {
                match self.handle_mcp_method(method, params.clone()).await {
                    Ok(result) => Ok(BusMessage::ok("tihc-mcp-tools-response", result)),
                    Err(e) => Ok(BusMessage::error("mcp-tools-error", 
                        format!("Method '{}' failed: {}", method, e)))
                }
            } else {
                Ok(BusMessage::error("invalid_format", 
                    "Expected {method, params} format"))
            }
        } else {
            Ok(BusMessage::error("parse_error", 
                "Failed to parse message data"))
        }
    }
}

/// 注册rmcp工具组件处理器到消息总线
pub async fn register_rmcp_tools_handler() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    
    REGISTERED.get_or_init(|| {
        let handler = Arc::new(RmcpToolsHandler::new());
        let bus = GLOBAL_MESSAGE_BUS.clone();
        
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                bus.register("tihc-mcp-tools", handler.clone(), HandlerMode::Request).await;
                tracing::info!("rmcp工具组件处理器已注册到消息总线 (topic: tihc-mcp-tools)");
            });
        });
    });
}
