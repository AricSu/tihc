use std::sync::Arc;

use microkernel::platform::PluginManager;
use plugin_causality_engine::plugin::CausalityEnginePluginDef;
use plugin_lossy_ddl::LossyDDLPlugin;
use tracing::info;

pub async fn register_plugins(manager: &mut PluginManager) {
    info!("🔧 [PLUGIN_MANAGER] Starting plugin registration process...");

    // 注册 Lossy DDL 插件
    info!("📦 [PLUGIN_MANAGER] Registering plugin_lossy_ddl...");
    manager.register_plugin(Arc::new(LossyDDLPlugin::new()));
    info!("✅ [PLUGIN_MANAGER] plugin_lossy_ddl registered successfully");

    // 注册因果分析引擎插件
    info!("📦 [PLUGIN_MANAGER] Registering plugin_causality_engine...");
    manager.register_plugin(Arc::new(CausalityEnginePluginDef::new()));
    info!("✅ [PLUGIN_MANAGER] plugin_causality_engine registered successfully");

    info!("🎉 [PLUGIN_MANAGER] All plugins registered successfully! Total: 2 plugins");

    // manager.register_plugin(Arc::new(SlowLogPlugin::new())).await;
    // manager.register_plugin(Arc::new(SqlEditorPlugin::new())).await;
    // manager.register_plugin(Arc::new(MultiplexerPlugin::new())).await;
}
