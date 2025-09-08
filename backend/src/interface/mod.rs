// Interface Layer - 接口层（原API层）
// HTTP controllers, 输入验证，响应格式化

pub mod http;
pub mod mcp;

// 重新导出接口适配器
pub use http::*;
pub use mcp::*;
