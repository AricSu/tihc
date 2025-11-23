// Domain 层：核心业务逻辑，不依赖其他层
// 只导出必要的公共接口，保持模块边界清晰

pub mod auth;
pub mod basic;
pub mod chat;
pub mod mcp;
pub mod plugins;
pub mod shared;

pub use chat::*;
pub use plugins::*;

// 只导出核心的共享类型和错误处理
// Re-export core domain types and services selectively
pub use shared::services::MessageBus;

// 导出认证相关的主要类型
pub use auth::{
    Claims, LoginResponse, Session, User, UserInfo, UserProvider, UserProviderRepository,
    UserRepository,
};

// 导出 DDL 检查的核心类型
pub use ddl_precheck::{
    DDLAnalysisResult, DDLPrecheckDomainService, DDLStatement, LossyStatus, RiskLevel,
};

// 导出 MCP 相关的域类型
pub use mcp::counter::Counter;
pub use mcp::lossy_ddl::LossyDdlRequest;
