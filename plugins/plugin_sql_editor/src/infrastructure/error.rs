#[macro_export]
macro_rules! try_lock {
    ($e:expr) => {
        $e.lock_safe().map_err(StoreError::from)?
    };
}

use common::CommonError;
use std::sync::PoisonError;
use thiserror::Error;

/// 扩展 Mutex，提供 lock_safe()，自动处理 PoisonError
use std::sync::{Mutex, MutexGuard};

pub trait MutexExt<T> {
    fn lock_safe(&self) -> Result<MutexGuard<'_, T>, CommonError>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn lock_safe(&self) -> Result<MutexGuard<'_, T>, CommonError> {
        self.lock().map_err(poison_to_common)
    }
}

/// SQL Editor infra 层错误类型，支持自动聚合和类型安全扩展
#[derive(Debug, Error)]
pub enum StoreError {
    /// 基础设施通用错误
    #[error("Infra error: {0}")]
    Infra(#[from] CommonError),
    /// IO 相关错误
    #[error("IO error: {0}")]
    Io(String),
    /// 配置相关错误
    #[error("Config error: {0}")]
    Config(String),
    /// 插件相关错误
    #[error("Plugin error: {0}")]
    Plugin(String),
    /// 外部依赖错误
    #[error("External error: {0}")]
    External(String),
    /// 其它 infra 错误
    #[error("Other infra error: {0}")]
    Other(String),
}

/// Convert PoisonError to CommonError::LockError (orphan rule safe)
pub fn poison_to_common<T>(e: PoisonError<T>) -> CommonError {
    CommonError::LockError(e.to_string())
}
