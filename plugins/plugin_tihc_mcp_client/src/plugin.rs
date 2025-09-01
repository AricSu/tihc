use microkernel::plugin_api::traits::{Plugin, PluginContext};

pub struct McpClientPlugin;

impl Plugin for McpClientPlugin {
    fn name(&self) -> &str {
        "plugin_tihc_mcp_client"
    }
    fn register(&mut self, _ctx: &mut PluginContext) {
        // 仅用于标记和依赖注入，无实际注册逻辑
    }
}