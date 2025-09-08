use async_trait::async_trait;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    model::*,
    service::RequestContext,
};
use serde_json::Value;

use crate::interface::mcp::registry::ComponentRegistry;
use crate::interface::mcp::dispatcher::RequestDispatcher;
use crate::interface::mcp::aggregator::CapabilityAggregator;

/// 统一API层 - 简化的MCP协议适配器
#[derive(Clone)]
pub struct UnifiedMcpAdapter {
    dispatcher: RequestDispatcher,
    aggregator: CapabilityAggregator,
}

impl UnifiedMcpAdapter {
    pub fn new() -> Self {
        let registry = ComponentRegistry::new();
        let dispatcher = RequestDispatcher::new(registry.clone());
        let aggregator = CapabilityAggregator::new(registry.clone());
        
        Self {
            dispatcher,
            aggregator,
        }
    }

    /// 简化的请求处理 - 返回空结果而不是默认值
    async fn handle_request_with_default<T>(&self, method: &str, params: Value, default: T) -> Result<T, McpError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        let request_data = serde_json::json!({
            "method": method,
            "params": params
        });

        match self.dispatcher.dispatch(method, request_data).await {
            Ok(result) => serde_json::from_value(result)
                .map_err(|e| McpError::internal_error(format!("Response parse error: {}", e), None)),
            Err(e) => {
                tracing::warn!("Request failed for {}: {}, using default", method, e);
                Ok(default)
            }
        }
    }
}

#[async_trait]
impl ServerHandler for UnifiedMcpAdapter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: self.aggregator.get_aggregated_capabilities(),
            server_info: Implementation::from_build_env(),
            instructions: Some("TiHC统一API层 - 支持多组件协作的MCP服务\n\n组件架构：\n• rmcp工具组件：提供工具调用和提示模板\n• SQL分析组件：提供数据库资源访问\n• 巡检逻辑组件：提供系统监控功能".to_string()),
        }
    }

    fn initialize(
        &self,
        request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<InitializeResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            let _ = self.dispatcher.dispatch("initialize", params).await;
            Ok(self.get_info())
        }
    }

    fn list_tools(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            let default = ListToolsResult { tools: vec![], next_cursor: None };
            self.handle_request_with_default("list_tools", params, default).await
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParam,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            match self.dispatcher.dispatch("call_tool", params).await {
                Ok(result) => serde_json::from_value(result)
                    .map_err(|e| McpError::internal_error(format!("Tool call parse error: {}", e), None)),
                Err(e) => Err(McpError::internal_error(format!("Tool execution failed: {}", e), None))
            }
        }
    }

    fn list_prompts(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            let default = ListPromptsResult { prompts: vec![], next_cursor: None };
            self.handle_request_with_default("list_prompts", params, default).await
        }
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            match self.dispatcher.dispatch("get_prompt", params).await {
                Ok(result) => serde_json::from_value(result)
                    .map_err(|e| McpError::internal_error(format!("Prompt parse error: {}", e), None)),
                Err(e) => Err(McpError::internal_error(format!("Prompt retrieval failed: {}", e), None))
            }
        }
    }

    fn list_resources(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            let default = ListResourcesResult { resources: vec![], next_cursor: None };
            self.handle_request_with_default("list_resources", params, default).await
        }
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            match self.dispatcher.dispatch("read_resource", params).await {
                Ok(result) => serde_json::from_value(result)
                    .map_err(|e| McpError::internal_error(format!("Resource read parse error: {}", e), None)),
                Err(e) => Err(McpError::resource_not_found(format!("Resource not found: {}", e), None))
            }
        }
    }

    fn list_resource_templates(
        &self,
        request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourceTemplatesResult, McpError>> + Send + '_ {
        async move {
            let params = serde_json::to_value(&request).unwrap_or(Value::Null);
            let default = ListResourceTemplatesResult { next_cursor: None, resource_templates: Vec::new() };
            self.handle_request_with_default("list_resource_templates", params, default).await
        }
    }
}
