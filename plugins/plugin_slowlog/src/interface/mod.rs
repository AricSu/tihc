// 微内核架构下，interface 层只做 trait re-export，不实现 handler/业务逻辑。
pub(crate) use super::application::slowlog_service::SlowLogService;
pub mod handler;
