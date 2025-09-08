// Infrastructure Layer - 基础设施层
// 外部系统集成，数据持久化，技术实现

pub mod database;
pub mod web;
pub mod logging;
pub mod config;

// 重新导出基础设施组件
pub use database::*;
pub use web::*;
pub use logging::*;
pub use config::*;
