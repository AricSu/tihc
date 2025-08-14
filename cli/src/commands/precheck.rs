use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct DDLCheckOptions {
    /// Path to the DDL SQL file to check (required)
    #[clap(long, short = 'f', value_name = "file", required = true)]
    pub file: String,
    /// Whether to enable collation support (default: true)
    #[clap(long, short = 'c', default_value_t = true)]
    pub collation: bool,
}

impl DDLCheckOptions {
    /// 读取文件内容，返回字符串
    pub fn read_sql_file(&self) -> Result<String> {
        std::fs::read_to_string(&self.file)
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", self.file, e))
    }
}
