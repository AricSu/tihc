// Infrastructure Layer - 基础设施层
// 外部系统集成，数据持久化，技术实现

pub mod bus_client;
pub mod config;
pub mod database;
pub mod extension;
pub mod logging;
pub mod web;

// 重新导出基础设施组件
pub use bus_client::*;
pub use config::*;
pub use database::*;
pub use extension::*;
pub use logging::*;
pub use web::*;
