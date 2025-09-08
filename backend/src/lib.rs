// Backend crate - 基于DDD架构的后端服务
// 遵循洋葱架构模式：Domain -> Application -> Infrastructure -> Interface

// 领域层 - 业务逻辑核心
pub mod domain;

// 应用层 - 用例编排和业务流程
pub mod application;

// 基础设施层 - 外部依赖和技术实现
pub mod infrastructure;

// 接口层 - API和外部接口
pub mod interface;

// 向后兼容的模块别名
pub mod api {
    pub use crate::interface::http::*;
    
    // 重新导出原有的路由创建函数
    pub fn create_router() -> axum::Router<crate::interface::http::controllers::AppState> {
        use axum::Router;
        
        Router::new()
            .merge(crate::interface::http::controllers::EditorController::routes())
            // TODO: 添加其他控制器路由
    }
}

pub mod handlers {
    
    pub mod static_files {
        pub use crate::interface::http::static_files::*;
    }
    
    // 这些handlers已经重构为DDD结构，不再直接导出
    // pub mod ddl_precheck; - 现在在 application::ddl_precheck 和 interface::http::ddl_controllers
    // pub mod editor_sql;   - 现在在 application::editor 和 interface::http::controllers
    // pub mod slowlog;      - 现在在 application::slowlog 和 interface::http::slowlog_controllers
}

pub mod server {
    // 服务器启动逻辑保持原位置
    pub use crate::infrastructure::web::server::*;
}

pub mod middleware {
    // 中间件保持原位置
    pub use crate::interface::http::middleware::*;
}
