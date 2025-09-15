use crate::domain::mcp::lossy_ddl::LossyDdlRequest;
use microkernel::platform::message_bus::BusMessage;
use microkernel::platform::message_bus::GLOBAL_MESSAGE_BUS;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    tool, tool_router, ErrorData as McpError,
};
use serde_json::json;

#[derive(Clone)]
pub struct LossyDdlService;

impl LossyDdlService {
    pub fn new() -> Self {
        Self
    }
}

#[tool_router]
impl LossyDdlService {
    #[tool(description = "检测 SQL 是否为危险 DDL")]
    pub async fn check_lossy_ddl(
        &self,
        Parameters(req): Parameters<LossyDdlRequest>,
    ) -> Result<CallToolResult, McpError> {
        let bus = GLOBAL_MESSAGE_BUS.clone();
        let topic = microkernel::topic!("ddl_precheck");
        let data = json!({ "sql": req.sql });
        // 这里用 request（异步等待返回）
        let result = bus.request(BusMessage::ok(topic, data), None).await;

        match result {
            Ok(reply) => {
                // reply.data 已是 Value 类型
                Ok(CallToolResult::success(vec![Content::json(reply.data)?]))
            }
            Err(e) => Err(McpError::from(rmcp::ErrorData {
                code: rmcp::model::ErrorCode::INTERNAL_ERROR,
                message: std::borrow::Cow::Owned(e.to_string()),
                data: None,
            })),
        }
    }
}
