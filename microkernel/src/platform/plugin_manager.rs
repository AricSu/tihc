//! PluginManager: 插件生命周期和 handler 管理，负责插件注册、卸载、查找和 handler 绑定。

use crate::platform::message_bus::{BusMessage};
use std::collections::HashMap;
use std::sync::Arc;

/// 插件 trait，所有插件只需实现元信息和生命周期相关方法。
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    /// 优雅关闭：收到 shutdown 消息时执行的操作
    fn on_shutdown(&self, msg: &BusMessage) -> anyhow::Result<()>;
    /// 插件声明需要注册的 topic 列表
    fn topics(&self) -> Vec<String>;
}

// 已不再需要 PluginHandler，所有 handler 注册请用 register_fn
/// PluginManager 负责插件的注册、卸载、查找和 handler 管理。
pub struct PluginManager {
    plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl PluginManager {
    /// 创建新的 PluginManager。
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    /// 注册插件对象，仅保存元信息，handler 注册请用 register_fn。
    pub fn register_plugin(&mut self, plugin: Arc<dyn Plugin>) {
        tracing::debug!(target: "plugin_manager", "Registering plugin: {}", plugin.name());
        let plugin_name = plugin.name().to_string();
        self.plugins.insert(plugin_name.clone(), plugin);
        tracing::debug!(target: "plugin_manager", "Plugin {} registered", plugin_name);
    }

    /// 卸载插件。
    pub fn unregister_plugin(&mut self, name: &str) {
        self.plugins.remove(name);
    }

    /// 按名称查找插件对象。
    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
        self.plugins.get(name)
    }

    /// 获取所有已注册插件名称。
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
