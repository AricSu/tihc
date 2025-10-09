//! 数据源统一抽象模块
//! 提供 HTTP 客户端基础能力和统一接口
//! 支持 Polars 数据处理

use async_trait::async_trait;
use polars::prelude::*;
use reqwest::{Client, header::HeaderMap, header::HeaderValue};
use std::collections::HashMap;
use tracing::{error, info};

/// 统一数据源配置
#[derive(Debug, Clone)]
pub enum DataSourceConfig {
    Prometheus {
        base_url: String,
        cookie: String,
    },
    Clinic {
        base_url: String,
        apikey: Option<String>,
        cookie: Option<String>,
        csrf_token: Option<String>,
    },
}

/// 统一查询结果，支持 Polars DataFrame
#[derive(Debug, Clone)]
pub enum QueryResult {
    Metrics {
        data: serde_json::Value,
    },
    Raw {
        raw_data: String,
    },
    Json {
        json: serde_json::Value,
    },
    DataFrame {
        df: DataFrame,
    },
    TimeSeries {
        df: DataFrame,
        timestamp_col: String,
        value_col: String,
    },
}

/// 连接测试结果
#[derive(Debug, Clone)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
}

/// 数据处理工具
pub struct DataProcessor;

impl DataProcessor {
    /// 将 Prometheus metrics 转换为 DataFrame
    pub fn prometheus_to_dataframe(data: &serde_json::Value) -> PolarsResult<DataFrame> {
        let mut timestamps = Vec::<i64>::new();
        let mut values = Vec::<f64>::new();
        let mut metric_names = Vec::<String>::new();
        let mut labels = Vec::<String>::new();

        if let Some(result) = data.as_array() {
            for metric in result {
                let metric_name = metric
                    .get("metric")
                    .and_then(|m| m.get("__name__"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown");

                let label_str = metric
                    .get("metric")
                    .map(|m| serde_json::to_string(m).unwrap_or_default())
                    .unwrap_or_default();

                if let Some(values_array) = metric.get("values").and_then(|v| v.as_array()) {
                    for value_pair in values_array {
                        if let Some(pair) = value_pair.as_array() {
                            if pair.len() >= 2 {
                                let timestamp = pair[0].as_f64().unwrap_or(0.0) as i64;
                                let value = pair[1]
                                    .as_str()
                                    .and_then(|s| s.parse::<f64>().ok())
                                    .unwrap_or(0.0);

                                timestamps.push(timestamp);
                                values.push(value);
                                metric_names.push(metric_name.to_string());
                                labels.push(label_str.clone());
                            }
                        }
                    }
                }
            }
        }

        df! {
            "timestamp" => timestamps,
            "value" => values,
            "metric_name" => metric_names,
            "labels" => labels,
        }
    }

    /// 将 Clinic 集群数据转换为 DataFrame
    pub fn clinic_cluster_to_dataframe(data: &serde_json::Value) -> PolarsResult<DataFrame> {
        let mut cluster_ids = Vec::<String>::new();
        let mut cluster_names = Vec::<String>::new();
        let mut statuses = Vec::<String>::new();
        let mut created_ats = Vec::<i64>::new();

        if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
            for item in items {
                cluster_ids.push(
                    item.get("clusterID")
                        .and_then(|id| id.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                );
                cluster_names.push(
                    item.get("clusterName")
                        .and_then(|name| name.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                );
                statuses.push(
                    item.get("clusterStatus")
                        .and_then(|status| status.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                );
                created_ats.push(
                    item.get("clusterCreatedAt")
                        .and_then(|ts| ts.as_i64())
                        .unwrap_or(0),
                );
            }
        }

        df! {
            "cluster_id" => cluster_ids,
            "cluster_name" => cluster_names,
            "status" => statuses,
            "created_at" => created_ats,
        }
    }

    /// 基础数据统计分析（简化版本）
    pub fn basic_stats(df: &DataFrame, column: &str) -> Result<HashMap<String, f64>, String> {
        let column_series = df
            .column(column)
            .map_err(|e| format!("Column '{}' not found: {}", column, e))?;

        let mut stats = HashMap::new();
        stats.insert("count".to_string(), column_series.len() as f64);

        // 基础统计 - 简化实现
        if column_series.len() > 0 {
            stats.insert("length".to_string(), column_series.len() as f64);
        }

        Ok(stats)
    }

    /// 简单异常检测（基于阈值）
    pub fn detect_anomalies_simple(
        _df: &DataFrame,
        _value_col: &str,
        _threshold: f64,
    ) -> Result<Vec<bool>, String> {
        // 占位符实现，后续可以添加具体的异常检测逻辑
        Ok(vec![false]) // 简化返回
    }
}

/// HTTP 客户端基础抽象
pub struct HttpDataSource {
    pub base_url: String,
    pub client: Client,
    pub default_headers: HeaderMap,
}

impl HttpDataSource {
    pub fn new(base_url: String, headers: HeaderMap) -> Self {
        let client_builder = Client::builder();
        let client = client_builder.build().unwrap();

        Self {
            base_url,
            client,
            default_headers: headers,
        }
    }

    /// 通用 GET 请求
    pub async fn get_request(
        &self,
        path: &str,
        query_params: Option<&[(&str, &str)]>,
        additional_headers: Option<HeaderMap>,
    ) -> Result<serde_json::Value, String> {
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );

        let mut request = self.client.get(&url);

        // 添加默认 headers
        for (key, value) in self.default_headers.iter() {
            request = request.header(key, value);
        }

        // 添加额外 headers
        if let Some(headers) = additional_headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }

        // 添加查询参数
        if let Some(params) = query_params {
            request = request.query(params);
        }

        info!("Sending request to: {}", url);

        let response = request
            .send()
            .await
            .map_err(|e| format!("Request error: {}", e))?;
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        serde_json::from_str(&body).map_err(|e| {
            error!("Failed to parse JSON: {}, body: {}", e, body);
            format!("JSON parse error: {}", e)
        })
    }

    /// 健康检查通用方法
    pub async fn health_check(&self, health_path: &str) -> Result<ConnectionTestResult, String> {
        match self.get_request(health_path, None, None).await {
            Ok(_) => {
                info!("Health check successful for {}", self.base_url);
                Ok(ConnectionTestResult {
                    success: true,
                    message: "Connection successful".to_string(),
                })
            }
            Err(e) => {
                error!("Health check failed for {}: {}", self.base_url, e);
                Ok(ConnectionTestResult {
                    success: false,
                    message: format!("Connection failed: {}", e),
                })
            }
        }
    }
}

/// 数据源通用 trait
#[async_trait]
pub trait DataSource: Send + Sync {
    async fn test_connection(&self) -> Result<ConnectionTestResult, String>;
    async fn query(
        &self,
        query: &str,
        params: Option<HashMap<&str, String>>,
    ) -> Result<QueryResult, String>;
}

/// 从配置创建数据源的工厂方法
pub fn create_data_source(config: DataSourceConfig) -> Box<dyn DataSource> {
    match config {
        DataSourceConfig::Prometheus { base_url, cookie } => {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Cookie",
                HeaderValue::from_str(&format!("grafana_session={}", cookie)).unwrap(),
            );
            let http_source = HttpDataSource::new(base_url, headers);
            Box::new(PrometheusDataSource { http_source })
        }
        DataSourceConfig::Clinic {
            base_url,
            apikey,
            cookie,
            csrf_token,
        } => {
            let mut headers = HeaderMap::new();
            if let Some(key) = apikey {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!("Bearer {}", key)).unwrap(),
                );
            } else if let (Some(cookie), Some(token)) = (cookie, csrf_token) {
                headers.insert("Cookie", HeaderValue::from_str(&cookie).unwrap());
                headers.insert("X-CSRF-Token", HeaderValue::from_str(&token).unwrap());
            }
            let http_source = HttpDataSource::new(base_url, headers);
            Box::new(ClinicDataSource { http_source })
        }
    }
}

/// Prometheus 数据源实现
pub struct PrometheusDataSource {
    http_source: HttpDataSource,
}

#[async_trait]
impl DataSource for PrometheusDataSource {
    async fn test_connection(&self) -> Result<ConnectionTestResult, String> {
        self.http_source
            .health_check("api/datasources/proxy/1/api/v1/status/config")
            .await
    }

    async fn query(
        &self,
        query: &str,
        params: Option<HashMap<&str, String>>,
    ) -> Result<QueryResult, String> {
        let mut query_params = vec![("query", query)];
        if let Some(params) = &params {
            for (key, value) in params {
                query_params.push((key, value));
            }
        }

        let result = self
            .http_source
            .get_request(
                "api/datasources/proxy/1/api/v1/query",
                Some(&query_params),
                None,
            )
            .await?;

        // 解析 Prometheus 特有格式并转换为 DataFrame
        if let Some(status) = result.get("status").and_then(|s| s.as_str()) {
            if status == "success" {
                if let Some(data) = result.get("data").and_then(|d| d.get("result")) {
                    match DataProcessor::prometheus_to_dataframe(data) {
                        Ok(df) => return Ok(QueryResult::DataFrame { df }),
                        Err(_) => return Ok(QueryResult::Metrics { data: data.clone() }),
                    }
                }
            }
        }

        Ok(QueryResult::Json { json: result })
    }
}

/// Clinic 数据源实现
pub struct ClinicDataSource {
    http_source: HttpDataSource,
}

#[async_trait]
impl DataSource for ClinicDataSource {
    async fn test_connection(&self) -> Result<ConnectionTestResult, String> {
        self.http_source
            .health_check("clinic/api/v1/dashboard/clusters2?limit=1")
            .await
    }

    async fn query(
        &self,
        query: &str,
        _params: Option<HashMap<&str, String>>,
    ) -> Result<QueryResult, String> {
        // Clinic 的 query 可以是 cluster_id 或其他查询标识
        let query_params = [("cluster_id", query), ("limit", "1")];

        let result = self
            .http_source
            .get_request(
                "clinic/api/v1/dashboard/clusters2",
                Some(&query_params),
                None,
            )
            .await?;

        // 尝试转换为 DataFrame
        match DataProcessor::clinic_cluster_to_dataframe(&result) {
            Ok(df) => Ok(QueryResult::DataFrame { df }),
            Err(_) => Ok(QueryResult::Json { json: result }),
        }
    }
}
