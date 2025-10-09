use anyhow::Result;
use clap::Args;
use microkernel::platform::message_bus::BusMessage;
use microkernel::platform::message_bus::GLOBAL_MESSAGE_BUS;

#[derive(Args, Debug)]
pub struct DDLCheckOptions {
    /// Path to the DDL SQL file to check (required)
    #[clap(long, short = 'f', value_name = "file", required = true)]
    pub sql_file: String,
    /// Whether to enable collation support (default: true)
    #[clap(long, short = 'c', default_value_t = true)]
    pub collation: bool,
}

impl DDLCheckOptions {
    /// 读取文件内容，返回字符串
    pub fn read_sql_file(&self) -> Result<String> {
        std::fs::read_to_string(&self.sql_file)
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", self.sql_file, e))
    }
}

pub async fn handle_ddlcheck(opts: &DDLCheckOptions) -> Result<()> {
    let sql = opts.read_sql_file()?;
    let topic = microkernel::topic!("ddl-precheck");
    let data = serde_json::json!({
        "sql": sql,
        "collation": opts.collation,
    });
    let bus_msg = BusMessage::ok(topic, data);
    let reply = GLOBAL_MESSAGE_BUS
        .request(bus_msg, Some(std::time::Duration::from_secs(5)))
        .await?;
    println!("DDL Precheck Result: {:?}", reply);
    Ok(())
}
