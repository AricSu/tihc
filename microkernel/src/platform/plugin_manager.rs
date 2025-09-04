//! PluginManager: 插件生命周期和 handler 管理，负责插件注册、卸载、查找和 handler 绑定。

use crate::platform::message_bus::GLOBAL_MESSAGE_BUS;
use crate::platform::message_bus::{BusMessage, HandlerMode, MessageBus, MessageHandler};
use std::collections::HashMap;
use std::sync::Arc;

/// 插件 trait，所有插件需实现 name/description/handler。
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    /// 统一处理消息总线消息（广播/请求均可）
    fn handle(&self, msg: &BusMessage, mode: HandlerMode) -> anyhow::Result<BusMessage>;
    /// 优雅关闭：收到 shutdown 消息时执行的操作
    fn on_shutdown(&self, msg: &BusMessage) -> anyhow::Result<()>;
    /// 插件声明需要注册的 topic 列表
    fn topics(&self) -> Vec<String>;
}

// 定义一个 handler 结构体，专门负责调用插件的 handle 方法
struct PluginHandler {
    plugin: Arc<dyn Plugin>,
    mode: HandlerMode,
}
#[async_trait::async_trait]
impl MessageHandler for PluginHandler {
    async fn handle(&self, msg: BusMessage) -> anyhow::Result<BusMessage> {
        self.plugin.handle(&msg, self.mode)
    }
}
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

    /// 注册插件对象，并自动注册 bus handler。
    pub async fn register_plugin(&mut self, plugin: Arc<dyn Plugin>) {
        tracing::debug!(target: "plugin_manager", "Registering plugin: {}", plugin.name());
        let plugin_name = plugin.name().to_string();
        for topic in plugin.topics() {
            tracing::debug!(target: "plugin_manager", "Registering broadcast handler for topic: {}", topic);
            let broadcast_handler = Arc::new(PluginHandler {
                plugin: plugin.clone(),
                mode: HandlerMode::Broadcast,
            });
            GLOBAL_MESSAGE_BUS
                .register(&topic, broadcast_handler, HandlerMode::Broadcast)
                .await;
            tracing::debug!(target: "plugin_manager", "Registering request handler for topic: {}", topic);
            let request_handler = Arc::new(PluginHandler {
                plugin: plugin.clone(),
                mode: HandlerMode::Request,
            });
            GLOBAL_MESSAGE_BUS
                .register(&topic, request_handler, HandlerMode::Request)
                .await;
        }
        self.plugins.insert(plugin_name.clone(), plugin);
        tracing::debug!(target: "plugin_manager", "Plugin {} registered and handlers bound", plugin_name);
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
