pub mod simple_client;
pub mod simple_plugin;

// 重新导出主要类型
pub use simple_client::{
    AutoflowClient, AutoflowConfig, AutoflowError, ChatRequest, ChatResponse, ChatStream,
    StreamChunk,
};

pub use simple_plugin::{create_autoflow_plugin, AutoflowPlugin};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_creation() {
        let config = AutoflowConfig::default();
        let _plugin = create_autoflow_plugin(config);
        // 基本的插件创建测试
    }
}
