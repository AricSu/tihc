

// ...existing code...

/// 路由 glue
pub fn routes(registry: std::sync::Arc<std::sync::Mutex<microkernel::platform::ServiceRegistry>>) -> axum::Router {
	use axum::{routing::{get, post, delete}, Extension};
	axum::Router::new()
		.route("/editor/connections", get(list_connections).post(create_connection))
	.route("/editor/connections/{id}", get(get_connection).post(update_connection).delete(delete_connection))
		.layer(Extension(registry))
}

use axum::{
	Extension, Router,
	extract::{Json, Path},
	routing::{delete, get, post},
};
use common::json_resp;
use microkernel::platform::ServiceRegistry;
use microkernel::platform::command_registry::CommandRegistry;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct DBConnectionRequest {
	pub id: u64,
	pub name: String,
	pub engine: String,
	pub host: String,
	pub port: u16,
	pub username: String,
	pub password: Option<String>,
	pub database: Option<String>,
	pub use_tls: bool,
	pub ca_cert_path: Option<String>,
	pub created_at: String,
}

async fn exec_cmd(
	registry: &Arc<ServiceRegistry>,
	cmd: &str,
	args: Vec<String>,
) -> serde_json::Value {
	if let Some(cmd_reg) = registry.resolve::<Box<CommandRegistry>>() {
		match cmd_reg.execute(cmd, &args).await {
			Ok(result) => serde_json::to_value(json_resp!(success, result)).unwrap(),
			Err(e) => serde_json::to_value(json_resp!(Failed, e.to_string())).unwrap(),
		}
	} else {
		serde_json::to_value(json_resp!(NotFound, Some("command registry not found"))).unwrap()
	}
}

pub async fn update_connection(
	Extension(registry): Extension<Arc<ServiceRegistry>>,
	Path(conn_id): Path<u64>,
	Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
	Json(
		exec_cmd(
			&registry,
			"editor-connections-update",
			vec![conn_id.to_string(), serde_json::to_string(&req).unwrap()],
		)
		.await,
	)
}

pub async fn get_connection(
	Extension(registry): Extension<Arc<ServiceRegistry>>,
	Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
	Json(
		exec_cmd(
			&registry,
			"editor-connections-get",
			vec![conn_id.to_string()],
		)
		.await,
	)
}

pub async fn list_connections(
	Extension(registry): Extension<Arc<ServiceRegistry>>,
) -> Json<serde_json::Value> {
	Json(exec_cmd(&registry, "editor-connections-list", vec![]).await)
}

pub async fn create_connection(
	Extension(registry): Extension<Arc<ServiceRegistry>>,
	Json(req): Json<DBConnectionRequest>,
) -> Json<serde_json::Value> {
	tracing::info!(target: "backend_api", "[create_connection] called, engine={}, host={}, port={}, user={}, db={}, use_tls={}, ca_cert_path={:?}", req.engine, req.host, req.port, req.username, req.database.as_deref().unwrap_or(""), req.use_tls, req.ca_cert_path);
	Json(
		exec_cmd(
			&registry,
			"editor-connections-create",
			vec![serde_json::to_string(&req).unwrap()],
		)
		.await,
	)
}

pub async fn delete_connection(
	Extension(registry): Extension<Arc<ServiceRegistry>>,
	Path(conn_id): Path<u64>,
) -> Json<serde_json::Value> {
	Json(
		exec_cmd(
			&registry,
			"editor-connections-delete",
			vec![conn_id.to_string()],
		)
		.await,
	)
}
