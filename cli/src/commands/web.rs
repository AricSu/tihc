//! Implements the 'web' command logic for tihc CLI.

use clap::Args;
use core::platform::command_registry::CommandRegistry;

#[derive(Args, Debug, Default)]
pub struct WebOptions {
    /// Port to listen on.
    #[clap(long, default_value = "5000", help = "Port to listen on")]
    pub port: u16,
    /// Address to bind to.
    #[clap(long, default_value = "127.0.0.1", help = "Address to bind to")]
    pub host: String,
}

pub async fn start_web_service(
    opts: &WebOptions,
    command_registry: CommandRegistry,
    shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> anyhow::Result<()> {
    backend::server::start_server_with_shutdown(opts.host.clone(), opts.port, command_registry, shutdown_rx).await
}
