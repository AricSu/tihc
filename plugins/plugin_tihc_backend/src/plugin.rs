use axum::{Router, routing::get, response::IntoResponse};
use microkernel::plugin_api::traits::{Plugin, PluginContext};
use microkernel::platform::command_registry::CommandHandler;
use async_trait::async_trait;
use serde_json::json;
use crate::api;

// 通过消息总线注册的 HTTP 命令处理器
pub struct HttpCommandHandler;

#[async_trait]
impl CommandHandler for HttpCommandHandler {
	async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
		// 这里只是示例，实际可根据 args 路由到 axum handler
		Ok(json!({ "msg": "Hello from plugin_tihc_backend via command!", "args": args }))
	}
}

pub struct TihcBackendPlugin;

impl Plugin for TihcBackendPlugin {
	fn name(&self) -> &str {
		"tihc_backend"
	}

	fn register(&mut self, ctx: &mut PluginContext) {
		// 注册 HTTP 命令到消息总线
		ctx.register_command("http_hello", Box::new(HttpCommandHandler));

		// 注册完整 API 路由到 service_registry，供主程序统一启动 HTTP 服务
		let registry = ctx.service_registry.clone();
		let app = api::create_router(registry);
		ctx.service_registry.lock().unwrap().register(Box::new(app));

		tracing::info!("TihcBackendPlugin registered, http_hello command and API router available");
	}
}
