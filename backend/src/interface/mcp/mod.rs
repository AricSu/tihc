// MCP接口模块
pub mod handlers;
pub mod service;
pub mod adapter;
pub mod registry;
pub mod dispatcher;
pub mod aggregator;


// 重新导出主要API
pub use handlers::*;
pub use service::*;
pub use registry::*;
pub use dispatcher::*;
pub use aggregator::*;
pub use adapter::UnifiedMcpAdapter;
