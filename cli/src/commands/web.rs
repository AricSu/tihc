//! Implements the 'web' command logic for tihc CLI.

use anyhow::Result;
use clap::Args;
#[derive(Args, Debug, Clone)]
pub struct WebOptions {
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    #[clap(long, default_value_t = 8080)]
    pub port: u16,
}

pub async fn start_web_service(opts: WebOptions) -> Result<()> {
    // 直接调用 backend 的统一启动入口
    backend::server::start_server_with_shutdown(opts.host.clone(), opts.port).await
}
