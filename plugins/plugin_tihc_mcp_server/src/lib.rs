pub mod plugin;

use std::sync::Arc;
use microkernel::platform::message_bus::{BusMessage, MessageHandler, MessageBus, GLOBAL_MESSAGE_BUS, HandlerMode};
use async_trait::async_trait;
use anyhow::Result;

use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    prompt, prompt_handler, prompt_router, schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct StructRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ExamplePromptArgs {
    /// A message to put in the prompt
    pub message: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CounterAnalysisArgs {
    /// The target value you're trying to reach
    pub goal: i32,
    /// Preferred strategy: 'fast' or 'careful'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Clone)]
pub struct Counter {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Counter>,
    prompt_router: PromptRouter<Counter>,
}

#[tool_router]
impl Counter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Increment the counter by 1")]
    async fn increment(&self) -> Result<CallToolResult, McpError> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Decrement the counter by 1")]
    async fn decrement(&self) -> Result<CallToolResult, McpError> {
        let mut counter = self.counter.lock().await;
        *counter -= 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Get the current counter value")]
    async fn get_value(&self) -> Result<CallToolResult, McpError> {
        let counter = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Say hello to the client")]
    fn say_hello(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("hello")]))
    }

    #[tool(description = "Repeat what you say")]
    fn echo(&self, Parameters(object): Parameters<JsonObject>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::Value::Object(object).to_string(),
        )]))
    }

    #[tool(description = "Calculate the sum of two numbers")]
    fn sum(
        &self,
        Parameters(StructRequest { a, b }): Parameters<StructRequest>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            (a + b).to_string(),
        )]))
    }
}

#[prompt_router]
impl Counter {
    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "example_prompt")]
    async fn example_prompt(
        &self,
        Parameters(args): Parameters<ExamplePromptArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        let prompt = format!(
            "This is an example prompt with your message here: '{}'",
            args.message
        );
        Ok(vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(prompt),
        }])
    }

    /// Analyze the current counter value and suggest next steps
    #[prompt(name = "counter_analysis")]
    async fn counter_analysis(
        &self,
        Parameters(args): Parameters<CounterAnalysisArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let strategy = args.strategy.unwrap_or_else(|| "careful".to_string());
        let current_value = *self.counter.lock().await;
        let difference = args.goal - current_value;

        let messages = vec![
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "I'll analyze the counter situation and suggest the best approach.",
            ),
            PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Current counter value: {}\nGoal value: {}\nDifference: {}\nStrategy preference: {}\n\nPlease analyze the situation and suggest the best approach to reach the goal.",
                    current_value, args.goal, difference, strategy
                ),
            ),
        ];

        Ok(GetPromptResult {
            description: Some(format!(
                "Counter analysis for reaching {} from {}",
                args.goal, current_value
            )),
            messages,
        })
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for Counter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides counter tools and prompts. Tools: increment, decrement, get_value, say_hello, echo, sum. Prompts: example_prompt (takes a message), counter_analysis (analyzes counter state with a goal).".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}

/// 轻量级MCP适配器 - 将消息总线消息转换为标准MCP请求
/// 然后直接使用Counter的完整ServerHandler实现
pub struct McpMessageBusAdapter {
    counter: Arc<Counter>,
}

impl McpMessageBusAdapter {
    pub fn new(counter: Arc<Counter>) -> Self {
        Self { counter }
    }
    
    async fn handle_with_routers(&self, method: &str, params: Value) -> Result<Value, McpError> {
        match method {
            "register" => {
                // 触发延迟注册
                register_unified_mcp_handler().await;
                Ok(serde_json::json!({"status": "registered", "message": "MCP handlers registered successfully"}))
            }
            "initialize" => {
                let info = self.counter.get_info();
                Ok(serde_json::to_value(info).unwrap())
            }
            "list_tools" => {
                let tools = self.counter.tool_router.list_all();
                let result = ListToolsResult { tools, next_cursor: None };
                Ok(serde_json::to_value(result).unwrap())
            }
            "call_tool" => {
                let request = serde_json::from_value::<CallToolRequestParam>(params)
                    .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
                
                // 直接调用Counter的#[tool]方法
                let result = match request.name.as_ref() {
                    "increment" => self.counter.increment().await?,
                    "decrement" => self.counter.decrement().await?,
                    "get_value" => self.counter.get_value().await?,
                    "say_hello" => self.counter.say_hello()?,
                    "echo" => {
                        let args = request.arguments.unwrap_or_default();
                        self.counter.echo(Parameters(args))?
                    }
                    "sum" => {
                        let args = request.arguments.unwrap_or_default();
                        let struct_req: StructRequest = serde_json::from_value(serde_json::Value::Object(args))
                            .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
                        self.counter.sum(Parameters(struct_req))?
                    }
                    _ => return Err(McpError::invalid_params(format!("Unknown tool: {}", request.name), None)),
                };
                Ok(serde_json::to_value(result).unwrap())
            }
            "list_prompts" => {
                let prompts = self.counter.prompt_router.list_all();
                let result = ListPromptsResult { prompts, next_cursor: None };
                Ok(serde_json::to_value(result).unwrap())
            }
            "get_prompt" => {
                let request = serde_json::from_value::<GetPromptRequestParam>(params)
                    .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
                
                // 手动处理提示逻辑
                let result = match request.name.as_ref() {
                    "example_prompt" => {
                        let message = request.arguments
                            .as_ref()
                            .and_then(|args| args.get("message"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Hello");
                        
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
                        
                        let current_value = { *self.counter.counter.lock().await };
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
                    _ => return Err(McpError::invalid_params(format!("Unknown prompt: {}", request.name), None)),
                };
                Ok(serde_json::to_value(result).unwrap())
            }
            "list_resources" => {
                let resources = vec![
                    self.counter._create_resource_text("str:////Users/to/some/path/", "cwd"),
                    self.counter._create_resource_text("memo://insights", "memo-name"),
                ];
                let result = ListResourcesResult { resources, next_cursor: None };
                Ok(serde_json::to_value(result).unwrap())
            }
            "read_resource" => {
                let request = serde_json::from_value::<ReadResourceRequestParam>(params)
                    .map_err(|e| McpError::invalid_params(e.to_string(), None))?;
                
                let result = match request.uri.as_str() {
                    "str:////Users/to/some/path/" => {
                        ReadResourceResult {
                            contents: vec![ResourceContents::text("/Users/to/some/path/", request.uri)],
                        }
                    }
                    "memo://insights" => {
                        let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                        ReadResourceResult {
                            contents: vec![ResourceContents::text(memo, request.uri)],
                        }
                    }
                    _ => return Err(McpError::resource_not_found("resource_not_found", Some(json!({"uri": request.uri})))),
                };
                Ok(serde_json::to_value(result).unwrap())
            }
            "list_resource_templates" => {
                let result = ListResourceTemplatesResult {
                    next_cursor: None, 
                    resource_templates: Vec::new()
                };
                Ok(serde_json::to_value(result).unwrap())
            }
            _ => Err(McpError::invalid_request(format!("Unsupported method: {}", method), None)),
        }
    }
}

#[async_trait]
impl MessageHandler for McpMessageBusAdapter {
    async fn handle(&self, msg: BusMessage) -> Result<BusMessage> {
        tracing::debug!("McpAdapter received message on topic: {}", msg.topic);

        if !msg.is_ok() {
            return Ok(BusMessage::error(&msg.topic, "Received error message"));
        }

        let request_data = msg.data.data.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Missing request data"))?;

        let method = request_data.get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let params = request_data.get("params")
            .cloned()
            .unwrap_or(Value::Null);

        tracing::debug!("Processing MCP method: {} with params: {:?}", method, params);
        
        // 使用简化的路由器调用，避免复杂的Context创建
        let result = match self.handle_with_routers(method, params).await {
            Ok(result) => result,
            Err(_e) => {
                // 对于不支持的方法或错误，返回错误消息
                return Ok(BusMessage::error(&msg.topic, format!("Method '{}' failed", method)));
            }
        };

        Ok(BusMessage::ok(&msg.topic, result))
    }
}

/// 注册轻量级MCP适配器到消息总线
pub async fn register_unified_mcp_handler() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    
    REGISTERED.get_or_init(|| {
        let counter = Arc::new(Counter::new());
        let adapter = Arc::new(McpMessageBusAdapter::new(counter));
        let bus = GLOBAL_MESSAGE_BUS.clone();
        
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                // 注册适配器到多个消息总线主题
                bus.register("tihc-mcp", adapter.clone(), HandlerMode::Request).await;
                bus.register("tihc-mcp-tools", adapter.clone(), HandlerMode::Request).await;
                bus.register("tihc-mcp-resources", adapter.clone(), HandlerMode::Request).await;
                
                tracing::info!("MCP适配器已注册到消息总线 (topics: tihc-mcp, tihc-mcp-tools, tihc-mcp-resources)");
            });
        });
    });
}