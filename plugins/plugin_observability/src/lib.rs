
use async_trait::async_trait;
use reqwest::{Client, header::COOKIE};
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub enum DataSourceConfig {
	Prometheus { base_url: String, cookie: String },
	// 可扩展其他数据源配置
}

#[derive(Debug, Clone)]
pub struct MetricsData {
	pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct LogData {
	pub logs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TableData {
	pub table: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ConfigData {
	pub config: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ProcessData {
	pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct K8sData {
	pub resource: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct SubsystemData {
	pub status: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ConnectionTestResult {
	pub success: bool,
	pub message: String,
}

#[derive(Debug, Clone)]
pub enum QueryResult {
	Metrics { data: serde_json::Value },
	Raw { raw_data: String },
}

// ================== Adapter Trait ==================

#[async_trait]
pub trait MetricsAdapter: Send + Sync {
	async fn fetch_data(&self, query: &str, params: Option<HashMap<&str, String>>) -> Result<QueryResult, String>;
	async fn fetch_data_range(&self, query: &str, params: Option<HashMap<&str, String>>) -> Result<QueryResult, String>;
	async fn test_connection(&self) -> Result<ConnectionTestResult, String>;
	async fn execute_query(&self, query: &str) -> Result<QueryResult, String>;
}

// ================== Prometheus Adapter ==================

pub struct PrometheusSource {
	pub base_url: String,
	pub client: Client,
	pub cookie: String,
}

impl PrometheusSource {
	pub fn new(config: DataSourceConfig) -> Self {
		let DataSourceConfig::Prometheus { base_url, cookie } = config;
		Self {
			base_url,
			client: reqwest::Client::new(),
			cookie,
		}
	}
}

#[async_trait]
impl MetricsAdapter for PrometheusSource {
	async fn fetch_data(
		&self,
		query: &str,
		params: Option<HashMap<&str, String>>,
	) -> Result<QueryResult, String> {
		let url = format!("{}/api/datasources/proxy/1/api/v1/query", self.base_url);

		info!("Sending instant query to Prometheus: URL={}, query={}, params={:?}", url, query, params);

		let mut query_params = vec![("query", query.to_string())];
		if let Some(params) = params {
			for (key, value) in params {
				query_params.push((key, value));
			}
		}

		let response = self
			.client
			.get(&url)
			.query(&query_params)
			.header(COOKIE, format!("grafana_session={}", self.cookie))
			.send()
			.await
			.map_err(|e| format!("Request error: {}", e))?;

		let body = response
			.text()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		info!("Prometheus response body: {}", body);

		match serde_json::from_str::<serde_json::Value>(&body) {
			Ok(json) => {
				if let Some(status) = json.get("status").and_then(|s| s.as_str()) {
					if status == "success" {
						if let Some(data) = json.get("data") {
							if let Some(result) = data.get("result") {
								info!("Parsed Prometheus metrics successfully");
								return Ok(QueryResult::Metrics {
									data: result.clone(),
								});
							}
						}
					}
				}
				Ok(QueryResult::Raw { raw_data: body })
			}
			Err(e) => {
				error!("Failed to parse JSON response: {}", e);
				Ok(QueryResult::Raw { raw_data: body })
			}
		}
	}

	async fn fetch_data_range(
		&self,
		query: &str,
		params: Option<HashMap<&str, String>>,
	) -> Result<QueryResult, String> {
		let url = format!("{}/api/datasources/proxy/1/api/v1/query_range", self.base_url);

		info!("Sending range query to Prometheus: URL={}, query={}, params={:?}", url, query, params);

		let mut query_params = vec![("query", query.to_string())];
		if let Some(params) = params {
			for (key, value) in params {
				query_params.push((key, value));
			}
		}

		let response = self
			.client
			.get(&url)
			.query(&query_params)
			.header(COOKIE, format!("grafana_session={}", self.cookie))
			.send()
			.await
			.map_err(|e| format!("Request error: {}", e))?;

		let body = response
			.text()
			.await
			.map_err(|e| format!("Failed to parse response: {}", e))?;

		info!("Prometheus range response body: {}", body);

		match serde_json::from_str::<serde_json::Value>(&body) {
			Ok(json) => {
				if let Some(status) = json.get("status").and_then(|s| s.as_str()) {
					if status == "success" {
						if let Some(data) = json.get("data") {
							if let Some(result) = data.get("result") {
								info!("Parsed Prometheus range metrics successfully");
								return Ok(QueryResult::Metrics {
									data: result.clone(),
								});
							}
						}
					}
				}
				Ok(QueryResult::Raw { raw_data: body })
			}
			Err(e) => {
				error!("Failed to parse JSON response: {}", e);
				Ok(QueryResult::Raw { raw_data: body })
			}
		}
	}

	async fn test_connection(&self) -> Result<ConnectionTestResult, String> {
		info!("Testing Prometheus connection: base_url={}", self.base_url);

		let health_url = format!("{}/api/datasources/proxy/1/api/v1/status/config", self.base_url);

		match self
			.client
			.get(&health_url)
			.header(COOKIE, format!("grafana_session={}", self.cookie))
			.send()
			.await
		{
			Ok(response) => {
				if response.status().is_success() {
					info!("Prometheus connection test successful");
					Ok(ConnectionTestResult {
						success: true,
						message: "Prometheus connection successful".to_string(),
					})
				} else {
					error!("Prometheus connection test failed with status: {}", response.status());
					Ok(ConnectionTestResult {
						success: false,
						message: format!("Connection failed with status: {}", response.status()),
					})
				}
			}
			Err(e) => {
				error!("Prometheus connection test failed: {}", e);
				Ok(ConnectionTestResult {
					success: false,
					message: format!("Connection failed: {}", e),
				})
			}
		}
	}

	async fn execute_query(&self, query: &str) -> Result<QueryResult, String> {
		self.fetch_data(query, None).await
	}
}

// ================== DataManager Trait ==================

pub trait DataManager: Send + Sync {
	fn get_metrics(&self, query: &str) -> Result<MetricsData, String>;
	fn get_logs(&self, filter: &str) -> Result<LogData, String>;
	fn get_system_table(&self, query: &str) -> Result<TableData, String>;
	fn get_config(&self, target: &str) -> Result<ConfigData, String>;
	fn get_process_args(&self, target: &str) -> Result<ProcessData, String>;
	fn get_k8s_resource(&self, query: &str) -> Result<K8sData, String>;
	fn get_k8s_subsystem(&self, query: &str) -> Result<SubsystemData, String>;
}
