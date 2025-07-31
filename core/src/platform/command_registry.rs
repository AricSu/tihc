/// CommandHandler trait: 所有命令处理器需实现此 trait。
/// 实现 Send + Sync 以支持多线程和插件微内核场景。
/// CommandHandler trait: 所有命令处理器需实现此 trait。
/// 实现 Send + Sync 以支持多线程和插件微内核场景。
pub trait CommandHandler: Send + Sync {
    /// 处理命令参数，返回 JSON 数据
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value>;
}

/// CommandRegistry: 命令注册与分发中心。
/// 支持命令注册、分发、查询、移除。
pub struct CommandRegistry {
    handlers: std::collections::HashMap<String, Box<dyn CommandHandler + Send + Sync>>,
}

impl CommandRegistry {
    /// 创建新的命令注册中心
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    /// 注册命令处理器（同名会覆盖）
    pub fn register(&mut self, name: &str, handler: Box<dyn CommandHandler + Send + Sync>) {
        self.handlers.insert(name.to_string(), handler);
    }

    /// 移除命令处理器，返回是否存在
    pub fn remove(&mut self, name: &str) -> bool {
        self.handlers.remove(name).is_some()
    }

    /// 判断命令是否已注册
    pub fn contains(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }

    /// 获取所有已注册命令名
    pub fn list_commands(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// 根据命令名分发执行，返回 JSON 数据
    /// 未找到命令时返回错误
    pub fn execute(&self, name: &str, args: &[String]) -> anyhow::Result<serde_json::Value> {
        if let Some(handler) = self.handlers.get(name) {
            handler.handle(args)
        } else {
            Err(anyhow::anyhow!("Command not found: {}", name))
        }
    }
}
