use std::sync::Arc;

use microkernel::platform::PluginManager;
use plugin_lossy_ddl::LossyDDLPlugin;

pub async fn register_plugins(manager: &mut PluginManager) {
    manager.register_plugin(Arc::new(LossyDDLPlugin::new()));
    // manager.register_plugin(Arc::new(SlowLogPlugin::new())).await;
    // manager.register_plugin(Arc::new(SqlEditorPlugin::new())).await;
    // manager.register_plugin(Arc::new(MultiplexerPlugin::new())).await;
}
