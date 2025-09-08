// 向后兼容的service.rs - 重新导出新的统一适配器

pub use crate::interface::mcp::adapter::UnifiedMcpAdapter;

// 保持原有API的兼容性
pub type SimpleMcpProxy = UnifiedMcpAdapter;
