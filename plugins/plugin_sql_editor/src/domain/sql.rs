use serde::{Deserialize, Serialize};
/// Message returned by the database engine (e.g., warning, notice).
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SqlMessage {
    /// Message level (e.g., WARNING, INFO).
    pub level: String,
    /// Message content.
    pub content: String,
}

/// SQL 执行结果
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SqlResult {
    /// 列名
    pub column_names: Vec<String>,
    /// 列类型名
    pub column_type_names: Vec<String>,
    /// 行数据
    pub rows: Vec<Vec<serde_json::Value>>,
    /// 行数
    pub rows_count: Option<u64>,
    /// 错误信息
    pub error: Option<String>,
    /// 执行耗时（毫秒）
    pub latency_ms: Option<u64>,
    /// SQL 语句
    pub statement: Option<String>,
    /// 附加消息
    pub messages: Option<Vec<SqlMessage>>,
}
