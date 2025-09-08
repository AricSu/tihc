use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rmcp::model::ServerCapabilities;

/// 组件注册中心
/// 
/// 职责：
/// 1. 管理系统中所有MCP组件的注册信息
/// 2. 提供组件查找和路由信息
/// 3. 维护组件的能力描述
#[derive(Clone)]
pub struct ComponentRegistry {
    components: Arc<RwLock<HashMap<String, ComponentInfo>>>,
}

/// 组件信息
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    /// 组件名称
    pub name: String,
    /// 组件描述
    pub description: String,
    /// 消息总线topic列表
    pub topics: Vec<String>,
    /// 支持的MCP方法列表
    pub methods: Vec<String>,
    /// 组件能力
    pub capabilities: ComponentCapabilities,
    /// 组件状态
    pub status: ComponentStatus,
}

/// 组件能力
#[derive(Debug, Clone, Default)]
pub struct ComponentCapabilities {
    pub tools: bool,
    pub prompts: bool,
    pub resources: bool,
    pub logging: bool,
}

/// 组件状态
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentStatus {
    /// 已注册但未初始化
    Registered,
    /// 已初始化并运行中
    Running,
    /// 暂停或不可用
    Paused,
    /// 发生错误
    Error(String),
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册组件
    pub fn register_component(&self, component: ComponentInfo) {
        let mut components = self.components.write().unwrap();
        tracing::info!(
            "Registering component '{}' with topics: {:?}",
            component.name,
            component.topics
        );
        components.insert(component.name.clone(), component);
    }

    /// 注册rmcp工具组件
    pub fn register_rmcp_tools_component(&self) {
        let component = ComponentInfo {
            name: "rmcp-tools".to_string(),
            description: "rmcp工具组件 - 提供#[tool]和#[prompt]标注的功能".to_string(),
            topics: vec!["tihc-mcp-tools".to_string()],
            methods: vec![
                "list_tools".to_string(),
                "call_tool".to_string(),
                "list_prompts".to_string(),
                "get_prompt".to_string(),
            ],
            capabilities: ComponentCapabilities {
                tools: true,
                prompts: true,
                resources: false,
                logging: false,
            },
            status: ComponentStatus::Registered,
        };
        self.register_component(component);
    }

    /// 注册SQL分析组件
    pub fn register_sql_analysis_component(&self) {
        let component = ComponentInfo {
            name: "sql-analysis".to_string(),
            description: "SQL分析组件 - 提供数据库资源访问和分析".to_string(),
            topics: vec!["tihc-mcp-resources".to_string()],
            methods: vec![
                "list_resources".to_string(),
                "read_resource".to_string(),
                "list_resource_templates".to_string(),
            ],
            capabilities: ComponentCapabilities {
                tools: false,
                prompts: false,
                resources: true,
                logging: false,
            },
            status: ComponentStatus::Registered,
        };
        self.register_component(component);
    }

    /// 注册巡检逻辑组件
    pub fn register_inspection_component(&self) {
        let component = ComponentInfo {
            name: "inspection".to_string(),
            description: "巡检逻辑组件 - 提供系统监控和健康检查".to_string(),
            topics: vec!["tihc-mcp-inspection".to_string()],
            methods: vec![
                "system_health".to_string(),
                "performance_metrics".to_string(),
                "alert_status".to_string(),
            ],
            capabilities: ComponentCapabilities {
                tools: true,
                prompts: true,
                resources: true,
                logging: true,
            },
            status: ComponentStatus::Registered,
        };
        self.register_component(component);
    }

    /// 根据方法名查找对应的topic
    pub fn find_topic_for_method(&self, method: &str) -> Option<String> {
        let components = self.components.read().unwrap();
        
        // 按优先级查找
        for (_, component) in components.iter() {
            if component.methods.contains(&method.to_string()) {
                if let Some(topic) = component.topics.first() {
                    return Some(topic.clone());
                }
            }
        }

        // 默认路由策略
        match method {
            "list_tools" | "call_tool" => Some("tihc-mcp-tools".to_string()),
            "list_prompts" | "get_prompt" => Some("tihc-mcp-tools".to_string()),
            "list_resources" | "read_resource" | "list_resource_templates" => {
                Some("tihc-mcp-resources".to_string())
            }
            _ => Some("tihc-mcp".to_string()),
        }
    }

    /// 获取所有运行中的组件
    pub fn get_running_components(&self) -> Vec<ComponentInfo> {
        let components = self.components.read().unwrap();
        components
            .values()
            .filter(|c| c.status == ComponentStatus::Running)
            .cloned()
            .collect()
    }

    /// 获取所有组件
    pub fn get_all_components(&self) -> Vec<ComponentInfo> {
        let components = self.components.read().unwrap();
        components.values().cloned().collect()
    }

    /// 更新组件状态
    pub fn update_component_status(&self, name: &str, status: ComponentStatus) {
        let mut components = self.components.write().unwrap();
        if let Some(component) = components.get_mut(name) {
            component.status = status;
            tracing::debug!("Updated component '{}' status to {:?}", name, component.status);
        }
    }

    /// 获取组件信息
    pub fn get_component(&self, name: &str) -> Option<ComponentInfo> {
        let components = self.components.read().unwrap();
        components.get(name).cloned()
    }

    /// 获取所有组件的topic列表（用于分发器）
    pub fn get_all_topics(&self) -> Vec<String> {
        let components = self.components.read().unwrap();
        let mut topics = Vec::new();
        
        for component in components.values() {
            topics.extend_from_slice(&component.topics);
        }
        
        // 去重
        topics.sort();
        topics.dedup();
        topics
    }

    /// 获取组件统计信息（用于分发器）
    pub fn get_component_info(&self) -> serde_json::Value {
        let components = self.components.read().unwrap();
        
        serde_json::json!({
            "total_components": components.len(),
            "running_components": components.values().filter(|c| c.status == ComponentStatus::Running).count(),
            "registered_components": components.values().filter(|c| c.status == ComponentStatus::Registered).count(),
            "components": components.values().map(|c| serde_json::json!({
                "name": c.name,
                "status": format!("{:?}", c.status),
                "topics": c.topics,
                "methods": c.methods,
            })).collect::<Vec<_>>()
        })
    }

    /// 聚合所有组件的能力
    pub fn aggregate_capabilities(&self) -> ServerCapabilities {
        let components = self.components.read().unwrap();
        
        let mut has_tools = false;
        let mut has_prompts = false;
        let mut has_resources = false;
        let mut has_logging = false;

        for component in components.values() {
            if component.status == ComponentStatus::Running || component.status == ComponentStatus::Registered {
                has_tools = has_tools || component.capabilities.tools;
                has_prompts = has_prompts || component.capabilities.prompts;
                has_resources = has_resources || component.capabilities.resources;
                has_logging = has_logging || component.capabilities.logging;
            }
        }

        // 根据组件能力构建 ServerCapabilities
        // 由于 rmcp 的 builder 类型系统限制，我们需要使用匹配模式
        match (has_tools, has_prompts, has_resources, has_logging) {
            (true, true, true, true) => ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .enable_logging()
                .build(),
            (true, true, true, false) => ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .build(),
            (true, true, false, false) => ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            (true, false, false, false) => ServerCapabilities::builder()
                .enable_tools()
                .build(),
            (false, true, false, false) => ServerCapabilities::builder()
                .enable_prompts()
                .build(),
            (false, false, true, false) => ServerCapabilities::builder()
                .enable_resources()
                .build(),
            (false, false, false, true) => ServerCapabilities::builder()
                .enable_logging()
                .build(),
            // 其他组合（简化版本，只处理常见情况）
            _ => ServerCapabilities::builder().build(),
        }
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        let registry = Self::new();
        
        // 注册默认组件
        registry.register_rmcp_tools_component();
        registry.register_sql_analysis_component();
        registry.register_inspection_component();
        
        registry
    }
}
