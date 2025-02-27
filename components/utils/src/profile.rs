use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use tracing::{error, info};

const HEAP_PATH_SUFFIX: &str = "/debug/pprof/heap";
const GOROUTINE_PATH: &str = "/debug/pprof/goroutine";
const PROFILE_PATH_PREFIX: &str = "/debug/pprof/profile";
const PD_API_PREFIX: &str = "/pd/api/v1";

use nix::sys::signal::Signal as NixSignal;

pub async fn collect_profile(
    base_url: &str,
    component: &str,
    collection_type: &str,
    seconds: u64,
) -> Result<()> {
    let collector = ProfileCollector::new(base_url);
    collector
        .collect_profile(component, collection_type, seconds)
        .await
}

// 新增信号发送函数
pub fn send_signal_to_pid(pid: u32, sig: NixSignal) -> Result<()> {
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), sig)
        .context("Failed to send signal")?;
    Ok(())
}

pub fn send_usr1(pid: u32) -> Result<()> {
    send_signal_to_pid(pid, NixSignal::SIGUSR1)
}

// 辅助函数：构建 profile 路径
#[derive(Debug)]
pub struct ProfileCollector {
    client: Client,
    base_url: String,
}

impl ProfileCollector {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: format!("http://{}", base_url),
        }
    }

    pub async fn collect_profile(
        &self,
        component: &str,
        collection_type: &str,
        seconds: u64,
    ) -> Result<()> {
        let api_paths = self.get_api_paths(component, collection_type, seconds)?;
        self.collect_all_profiles(&api_paths).await
    }

    fn get_api_paths(
        &self,
        component: &str,
        collection_type: &str,
        seconds: u64,
    ) -> Result<Vec<String>> {
        let prefix = match component {
            "tidb" | "tikv" => "",
            "pd" => PD_API_PREFIX,
            _ => return Err(anyhow!("Unsupported component: {}", component)),
        };

        let mut paths = Vec::new();
        match collection_type {
            "heap" => paths.push(format!("{}{}", prefix, HEAP_PATH_SUFFIX)),
            "goroutine" if component == "tidb" => paths.push(GOROUTINE_PATH.to_string()),
            "profile" => paths.push(self.build_profile_path(prefix, seconds)),
            "all" => {
                paths.push(format!("{}{}", prefix, HEAP_PATH_SUFFIX));
                if component == "tidb" {
                    paths.push(GOROUTINE_PATH.to_string());
                }
                paths.push(self.build_profile_path(prefix, seconds));
            }
            _ => {
                return Err(anyhow!(
                    "Unsupported collection type for {}: {}",
                    component,
                    collection_type
                ))
            }
        }
        Ok(paths)
    }

    fn build_profile_path(&self, prefix: &str, seconds: u64) -> String {
        format!("{}{}", prefix, PROFILE_PATH_PREFIX)
    }

    async fn collect_all_profiles(&self, api_paths: &[String]) -> Result<()> {
        let mut errors = Vec::new();

        for path in api_paths {
            let profile_url = format!("{}{}", self.base_url, path);
            if let Err(e) = self.download_and_save_profile(&profile_url).await {
                error!("处理 profile 数据失败: {}", e);
                errors.push(format!("{}: {}", profile_url, e));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow!(errors.join("\n")))
        }
    }

    async fn download_and_save_profile(&self, url: &str) -> Result<()> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "获取 profile 数据失败，状态码: {}",
                response.status()
            ));
        }

        let profile_data = response.bytes().await?.to_vec();
        let file_name = self.generate_profile_filename(url);

        let mut file = File::create(&file_name)?;
        file.write_all(&profile_data)?;

        info!("性能分析数据已保存到文件: {}", file_name);
        Ok(())
    }

    fn generate_profile_filename(&self, url: &str) -> String {
        let replaced_path = url.replace("/", "_");
        let file_name = replaced_path.trim_start_matches("_");
        format!("{}.pb.gz", file_name)
    }
}
