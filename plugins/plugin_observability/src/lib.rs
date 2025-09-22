pub mod clinic_api;
pub mod data_source;

// 重新导出统一抽象
pub use data_source::{
    DataSourceConfig, QueryResult, ConnectionTestResult, 
    DataSource, HttpDataSource, create_data_source, DataProcessor
};

// 保留简化的类型别名，用于向后兼容
pub type MetricsData = QueryResult;
pub type LogData = QueryResult;
pub type TableData = QueryResult;
pub type ConfigData = QueryResult;
pub type ProcessData = QueryResult;
pub type K8sData = QueryResult;
pub type SubsystemData = QueryResult;

// ================== 统一数据管理器 ==================

use async_trait::async_trait;
use std::collections::HashMap;

/// 统一的数据管理器，基于新的 DataSource 架构
pub struct UnifiedDataManager {
    data_sources: HashMap<String, Box<dyn DataSource>>,
}

impl UnifiedDataManager {
    pub fn new() -> Self {
        Self {
            data_sources: HashMap::new(),
        }
    }

    /// 添加数据源
    pub fn add_data_source(&mut self, name: String, config: DataSourceConfig) {
        let data_source = create_data_source(config);
        self.data_sources.insert(name, data_source);
    }

    /// 获取数据源
    pub fn get_data_source(&self, name: &str) -> Option<&dyn DataSource> {
        self.data_sources.get(name).map(|ds| ds.as_ref())
    }

    /// 测试所有数据源连接
    pub async fn test_all_connections(&self) -> HashMap<String, ConnectionTestResult> {
        let mut results = HashMap::new();
        for (name, data_source) in &self.data_sources {
            match data_source.test_connection().await {
                Ok(result) => {
                    results.insert(name.clone(), result);
                }
                Err(e) => {
                    results.insert(name.clone(), ConnectionTestResult {
                        success: false,
                        message: format!("Connection test failed: {}", e),
                    });
                }
            }
        }
        results
    }

    /// 统一查询接口
    pub async fn query(
        &self, 
        source_name: &str, 
        query: &str, 
        params: Option<HashMap<&str, String>>
    ) -> Result<QueryResult, String> {
        match self.data_sources.get(source_name) {
            Some(data_source) => data_source.query(query, params).await,
            None => Err(format!("Data source '{}' not found", source_name)),
        }
    }
}

impl Default for UnifiedDataManager {
    fn default() -> Self {
        Self::new()
    }
}

// ================== 向后兼容的 DataManager Trait ==================

/// @deprecated 使用 UnifiedDataManager 和 DataSource trait 替代
#[async_trait]
pub trait DataManager: Send + Sync {
    async fn get_metrics(&self, query: &str) -> Result<QueryResult, String>;
    async fn get_logs(&self, filter: &str) -> Result<QueryResult, String>;
    async fn get_system_table(&self, query: &str) -> Result<QueryResult, String>;
    async fn get_config(&self, target: &str) -> Result<QueryResult, String>;
    async fn get_process_args(&self, target: &str) -> Result<QueryResult, String>;
    async fn get_k8s_resource(&self, query: &str) -> Result<QueryResult, String>;
    async fn get_k8s_subsystem(&self, query: &str) -> Result<QueryResult, String>;
}

/// 为 UnifiedDataManager 提供向后兼容的 DataManager 实现
#[async_trait]
impl DataManager for UnifiedDataManager {
    async fn get_metrics(&self, query: &str) -> Result<QueryResult, String> {
        // 默认使用 prometheus 数据源
        self.query("prometheus", query, None).await
    }

    async fn get_logs(&self, filter: &str) -> Result<QueryResult, String> {
        // 可以扩展支持 logs 数据源
        self.query("logs", filter, None).await
    }

    async fn get_system_table(&self, query: &str) -> Result<QueryResult, String> {
        // 使用 clinic 数据源查询系统表
        self.query("clinic", query, None).await
    }

    async fn get_config(&self, target: &str) -> Result<QueryResult, String> {
        self.query("config", target, None).await
    }

    async fn get_process_args(&self, target: &str) -> Result<QueryResult, String> {
        self.query("process", target, None).await
    }

    async fn get_k8s_resource(&self, query: &str) -> Result<QueryResult, String> {
        self.query("k8s", query, None).await
    }

    async fn get_k8s_subsystem(&self, query: &str) -> Result<QueryResult, String> {
        self.query("k8s_subsystem", query, None).await
    }
}
