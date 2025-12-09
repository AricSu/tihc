use crate::domain::shared::DomainResult;


/// 密码服务接口：抽象密码哈希和验证
pub trait PasswordService: Send + Sync {
    /// 哈希密码
    fn hash_password(&self, password: &str) -> DomainResult<String>;

    /// 验证密码
    fn verify_password(&self, password: &str, hash: &str) -> DomainResult<bool>;
}

/// UUID生成服务接口
pub trait UuidService: Send + Sync {
    fn generate(&self) -> String;
}
