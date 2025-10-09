// DDD结构下 lib.rs 仅做模块导入和导出
pub mod domain {
    pub mod model;
    pub mod service;
}
pub mod application {
    pub mod engine;
}
pub mod infrastructure {
    pub mod metrics;
}
pub mod plugin;
