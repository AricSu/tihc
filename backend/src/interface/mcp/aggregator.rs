use rmcp::model::ServerCapabilities;

use crate::interface::mcp::registry::ComponentRegistry;

/// 能力聚合器
/// 
/// 职责：
/// 1. 聚合所有组件的MCP能力信息
/// 2. 提供统一的能力视图
/// 3. 动态更新能力信息
#[derive(Clone)]
pub struct CapabilityAggregator {
    registry: ComponentRegistry,
}

impl CapabilityAggregator {
    pub fn new(registry: ComponentRegistry) -> Self {
        Self { registry }
    }

    /// 获取聚合后的服务器能力
    pub fn get_aggregated_capabilities(&self) -> ServerCapabilities {
        self.registry.aggregate_capabilities()
    }

    /// 获取能力详细信息
    pub fn get_capability_details(&self) -> CapabilityDetails {
        let components = self.registry.get_all_components();
        
        let mut tools_providers = Vec::new();
        let mut prompts_providers = Vec::new();
        let mut resources_providers = Vec::new();
        let mut logging_providers = Vec::new();

        for component in components {
            if component.capabilities.tools {
                tools_providers.push(component.name.clone());
            }
            if component.capabilities.prompts {
                prompts_providers.push(component.name.clone());
            }
            if component.capabilities.resources {
                resources_providers.push(component.name.clone());
            }
            if component.capabilities.logging {
                logging_providers.push(component.name.clone());
            }
        }

        CapabilityDetails {
            tools_providers,
            prompts_providers,
            resources_providers,
            logging_providers,
        }
    }

    /// 检查是否支持指定能力
    pub fn supports_capability(&self, capability: &str) -> bool {
        let capabilities = self.get_aggregated_capabilities();
        
        match capability {
            "tools" => capabilities.tools.is_some(),
            "prompts" => capabilities.prompts.is_some(),
            "resources" => capabilities.resources.is_some(),
            "logging" => capabilities.logging.is_some(),
            _ => false,
        }
    }

    /// 获取能力提供者
    pub fn get_capability_providers(&self, capability: &str) -> Vec<String> {
        let details = self.get_capability_details();
        
        match capability {
            "tools" => details.tools_providers,
            "prompts" => details.prompts_providers,
            "resources" => details.resources_providers,
            "logging" => details.logging_providers,
            _ => Vec::new(),
        }
    }
}

/// 能力详细信息
#[derive(Debug, Clone)]
pub struct CapabilityDetails {
    pub tools_providers: Vec<String>,
    pub prompts_providers: Vec<String>,
    pub resources_providers: Vec<String>,
    pub logging_providers: Vec<String>,
}

impl CapabilityDetails {
    /// 获取所有能力的总结
    pub fn summary(&self) -> String {
        format!(
            "MCP组件能力总结:\n• 工具提供者: {:?}\n• 提示模板提供者: {:?}\n• 资源提供者: {:?}\n• 日志提供者: {:?}",
            self.tools_providers,
            self.prompts_providers,
            self.resources_providers,
            self.logging_providers
        )
    }

    /// 检查是否有任何能力提供者
    pub fn has_any_providers(&self) -> bool {
        !self.tools_providers.is_empty()
            || !self.prompts_providers.is_empty()
            || !self.resources_providers.is_empty()
            || !self.logging_providers.is_empty()
    }
}
