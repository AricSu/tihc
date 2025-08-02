use thiserror::Error;

/// 通用基础设施错误类型，支持 DDD/微内核分层自动聚合
#[derive(Debug, Error)]
pub enum CommonError {
    /// 线程锁相关错误
    #[error("Common lock error: {0}")]
    LockError(String),
    /// IO 相关错误
    #[error("Common IO error: {0}")]
    IoError(String),
    /// 配置相关错误
    #[error("Common config error: {0}")]
    ConfigError(String),
    /// 插件相关错误
    #[error("Common plugin error: {0}")]
    PluginError(String),
    /// 外部依赖错误
    #[error("Common external error: {0}")]
    ExternalError(String),
    /// 其它通用错误
    #[error("Common error: {0}")]
    Other(String),
}

// Conversion from std::io::Error
impl From<std::io::Error> for CommonError {
    fn from(e: std::io::Error) -> Self {
        CommonError::IoError(e.to_string())
    }
}
