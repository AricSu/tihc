// Application Layer - 应用服务层
// 编排业务流程，协调领域对象，处理用例

pub mod editor;
pub mod database;
pub mod ddl_precheck;
pub mod slowlog;
pub mod mcp;
pub mod settings;
pub mod notifications;
pub mod table;
pub mod sql_editor;

// 应用服务trait定义
pub mod services;

// 重新导出应用服务
pub use services::*;
