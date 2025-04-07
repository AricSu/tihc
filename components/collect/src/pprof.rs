use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use tracing::info;

#[async_trait]
pub trait ProfileCollector {
    async fn collect(&self) -> Result<()>;
}

pub struct HttpProfileCollector {
    client: Client,
    base_url: String,
    component: String,
    collection_type: String,
    seconds: u64,
    path: String,
}

impl HttpProfileCollector {
    pub fn new(
        base_url: impl Into<String>,
        component: impl Into<String>,
        collection_type: impl Into<String>,
        seconds: u64,
        path: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            component: component.into(),
            collection_type: collection_type.into(),
            seconds,
            path: path.into(),
        }
    }

    async fn download_profile(&self, url: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("HTTP error: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP {} for {}", response.status(), url));
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| anyhow!("Failed to get bytes: {}", e))
    }

    pub fn get_profile_path(&self) -> Result<String> {
        let base = if self.component == "pd" {
            "/pd/api/v1"
        } else {
            ""
        };

        let endpoint = match self.collection_type.as_str() {
            "config" => return Ok(format!("{}/config", base)),
            "profile" => "profile",
            "mutex" => "mutex",
            "heap" => "heap",
            "goroutine" => "goroutine",
            _ => return Err(anyhow!("Invalid collection_type: {}. Valid values are: config, profile, mutex, heap, goroutine", self.collection_type)),
        };

        let query = if self.collection_type == "goroutine" {
            "?debug=2".to_string()
        } else {
            format!("?seconds={}", self.seconds)
        };

        Ok(format!("{}/debug/pprof/{}{}", base, endpoint, query))
    }
}

#[async_trait]
impl ProfileCollector for HttpProfileCollector {
    async fn collect(&self) -> Result<()> {
        let url = format!("http://{}{}", self.base_url, self.get_profile_path()?);
        let data = self
            .download_profile(&url)
            .await
            .map_err(|e| anyhow!("Failed to download {}: {}", url, e))?;

        let file_path = if self.path.is_empty() {
            format!(
                "tihc_{}_{}_{}.pprof",
                self.component,
                self.collection_type,
                Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string()
            )
        } else {
            self.path.to_string()
        };

        std::fs::write(&file_path, data).map_err(|e| anyhow!("Failed to save {}: {}", url, e))?;
        info!("Saved profile to {}", file_path);
        Ok(())
    }
}
