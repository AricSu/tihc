// Domain Layer - 业务逻辑核心层
// 包含所有业务实体、值对象、领域服务和聚合根

pub mod editor;
pub mod database_connection;
pub mod ddl_precheck;
pub mod slowlog;
pub mod mcp;
pub mod settings;
pub mod notifications;
pub mod table;
pub mod shared;

// 重新导出主要的领域类型
pub use editor::*;
pub use database_connection::*;
pub use notifications::*;
pub use table::*;
pub use ddl_precheck::*;
pub use slowlog::*;
pub use mcp::*;
pub use settings::*;
pub use shared::*;
