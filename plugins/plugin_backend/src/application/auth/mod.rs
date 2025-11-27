pub mod auth_service;
pub mod dtos;
pub mod error_mapper;
pub mod oauth_service;
pub mod role_service;
pub mod user_service;

pub use auth_service::*;
pub use error_mapper::*;
pub use oauth_service::*;
pub use role_service::*;
pub use user_service::*;

// 移除了具体实现的类型别名，应该在组装层（接口层）进行依赖注入
