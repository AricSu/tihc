use crate::domain::shared::DomainResult;
use async_trait::async_trait;
use serde_json::Value;

/// 消息总线抽象接口：定义插件间通信契约
#[async_trait]
pub trait MessageBus: Send + Sync {
    /// 发送请求并等待响应
    async fn request(&self, topic: &str, data: Value) -> DomainResult<Value>;

    /// 发布消息（不等待响应）
    async fn publish(&self, topic: &str, data: Value) -> DomainResult<()>;

    /// 订阅主题
    async fn subscribe(&self, topic: &str) -> DomainResult<()>;
}

/// 密码服务接口：抽象密码哈希和验证
pub trait PasswordService: Send + Sync {
    /// 哈希密码
    fn hash_password(&self, password: &str) -> DomainResult<String>;

    /// 验证密码
    fn verify_password(&self, password: &str, hash: &str) -> DomainResult<bool>;
}

/// UUID生成服务接口
pub trait UuidService: Send + Sync {
    /// 生成新的UUID
    fn generate(&self) -> String;
}

/// HTTP客户端服务接口
#[async_trait]
pub trait HttpClientService: Send + Sync {
    /// 发送POST请求
    async fn post_form(
        &self,
        url: &str,
        params: &[(&str, &str)],
    ) -> DomainResult<serde_json::Value>;

    /// 发送带认证的GET请求
    async fn get_with_auth(&self, url: &str, token: &str) -> DomainResult<serde_json::Value>;
}

/// DDL 分析服务接口：定义 DDL 检查的外部分析能力
#[async_trait]
pub trait DDLAnalysisService: Send + Sync {
    /// 分析 DDL 语句的风险
    async fn analyze_ddl(
        &self,
        sql: &str,
        collation_enabled: bool,
    ) -> DomainResult<ExternalAnalysisResult>;
}

/// 外部分析结果：来自插件系统的分析结果
#[derive(Debug, Clone)]
pub struct ExternalAnalysisResult {
    pub lossy_status: String,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

impl ExternalAnalysisResult {
    pub fn new(lossy_status: String, warnings: Vec<String>, error: Option<String>) -> Self {
        Self {
            lossy_status,
            warnings,
            error,
        }
    }
}
