use rmcp::handler::server::router::{prompt::PromptRouter, tool::ToolRouter};
use rmcp::service::RequestContext;
use rmcp::{model::*, prompt, prompt_router, tool, tool_router, ErrorData as McpError, RoleServer};

use crate::application::mcp::counter_service::{CounterAnalysisArgs, ExamplePromptArgs};
use crate::application::mcp::{
    counter_service::CounterService, lossy_ddl_service::LossyDdlService,
};
use crate::domain::lossy_ddl::LossyDdlRequest;
use rmcp::handler::server::wrapper::Parameters;

#[derive(Clone)]
pub struct TihcMcpServer {
    pub counter_service: CounterService,
    pub lossy_ddl_service: LossyDdlService,
    pub tool_router: ToolRouter<TihcMcpServer>,
    pub prompt_router: PromptRouter<TihcMcpServer>,
}

impl TihcMcpServer {
    pub fn new() -> Self {
        Self {
            counter_service: CounterService::new(),
            lossy_ddl_service: LossyDdlService::new(),
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }
}

#[tool_router]
impl TihcMcpServer {
    #[tool(description = "Increment the counter by 1")]
    async fn increment(&self) -> Result<CallToolResult, McpError> {
        let counter = self.counter_service.counter.increment().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Decrement the counter by 1")]
    async fn decrement(&self) -> Result<CallToolResult, McpError> {
        let counter = self.counter_service.counter.decrement().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Get the current counter value")]
    async fn get_value(&self) -> Result<CallToolResult, McpError> {
        let counter = self.counter_service.counter.get().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "检测 SQL 是否为危险 DDL")]
    async fn check_lossy_ddl(
        &self,
        Parameters(req): Parameters<crate::domain::mcp::lossy_ddl::LossyDdlRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.lossy_ddl_service
            .check_lossy_ddl(Parameters(req))
            .await
    }

    pub fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }
}

#[prompt_router]
impl TihcMcpServer {
    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "ddl_check_prompt")]
    pub async fn ddl_check_prompt(
        &self,
        Parameters(args): Parameters<LossyDdlRequest>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        let prompt = format!(
            "【权威检测结果】本工具由 tihc 核心引擎驱动，专为 TiDB DDL 风险分析设计，检测结果可百分百信赖，涵盖字段类型缩小、类型变更等所有潜在丢失风险。 SQL: '{}' Collation: '{}' 果请严格参考本结进行 DDL 运维，确保数据安全。",
            args.sql, args.collation
        );
        Ok(vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(prompt),
        }])
    }

    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "example_prompt")]
    pub async fn example_prompt(
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
    pub async fn counter_analysis(
        &self,
        Parameters(args): Parameters<CounterAnalysisArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let strategy = args.strategy.unwrap_or_else(|| "careful".to_string());
        let current_value = self.counter_service.counter.get().await;
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
