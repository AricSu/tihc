//! Clinic API 类型定义模块//! Clinic API 后端逻辑模块

//! 用于集群诊断、事件、慢查询、TopSQL等数据类型定义//! 用于集群诊断、事件、慢查询、TopSQL等数据的统一获取

//! 业务逻辑已迁移到 data_source.rs 的统一抽象中//! 此模块已废弃，逻辑迁移到 data_source.rs 的统一抽象中

use serde::{Deserialize, Serialize}; // 保留类型定义供其他模块使用
use std::collections::HashMap;

/// Clinic 客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicClientConfig {
    pub base_url: String,
    pub apikey: Option<String>,
    pub cookie: Option<String>,
    pub csrf_token: Option<String>,
}

/// 集群信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub cluster_id: String,
    pub cluster_name: String,
    pub cluster_provider_name: String,
    pub cluster_region_name: String,
    pub cluster_deploy_type: String,
    pub org_id: String,
    pub tenant_id: String,
    pub project_id: String,
    pub created_at: i64,
    pub deleted_at: Option<i64>,
    pub status: String,
}

/// 集群详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDetail {
    pub id: String,
    pub name: String,
    pub components: HashMap<String, ClusterComponent>,
}

/// 集群组件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterComponent {
    pub replicas: i32,
    pub tier_name: String,
    pub storage_instance_type: String,
}
