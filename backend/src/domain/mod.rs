// Domain Layer - 业务逻辑核心层
// 包含所有业务实体、值对象、领域服务和聚合根

pub mod database_connection;
pub mod ddl_precheck;
pub mod editor;
pub mod extension;
pub mod inspection;
pub mod mcp;
pub mod notifications;
pub mod settings;
pub mod shared;
pub mod slowlog;
pub mod table;

// 重新导出主要的领域类型
pub use database_connection::*;
pub use ddl_precheck::*;
pub use editor::*;
pub use extension::*;
pub use inspection::{InspectionRequest, InspectionResponse, InspectionTask, TaskStatus};
pub use mcp::*;
pub use notifications::*;
pub use settings::*;
pub use shared::*;
pub use slowlog::*;
pub use table::*;
