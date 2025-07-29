use crate::domain::table::SlowQueryRow;
use crate::infrastructure::retriever::parse_log;
use anyhow::{Context, Result};
use futures::{Stream, stream};
use globset::Glob;
use std::future::Future;
use std::io::BufRead;
use std::pin::Pin;
use std::{fs, io};

pub(crate) trait SlowLogService: Send + Sync {
    fn parse_and_import<'a>(
        &'a self,
        log_dir: &'a str,
        pattern: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>>;
    fn scan_files<'a>(
        &'a self,
        dir: &'a str,
        pattern: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<String>>> + Send + 'a>>;
    fn parse_slow_log(
        self: Box<Self>,
        sender: tokio::sync::mpsc::Sender<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn data_for_slow_log(
        &self,
        receiver: tokio::sync::mpsc::Receiver<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<SlowQueryRow>>> + Send>>;
}

#[derive(Clone)]
pub(crate) struct SlowLogServiceImpl {
    batch_size: usize,
    file_paths: Vec<String>,
}

impl SlowLogServiceImpl {
    /// 扫描 slowlog 文件列表（仅供 web 前端调用，CLI 不应使用）
    pub(crate) fn scan_slowlog_files(dir: &str, pattern: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let glob = Glob::new(pattern)
            .map_err(|e| anyhow::anyhow!(format!("Invalid glob pattern: {}: {}", pattern, e)))?;
        let matcher = glob.compile_matcher();
        let entries = std::fs::read_dir(dir)
            .map_err(|e| anyhow::anyhow!(format!("Failed to read dir {}: {}", dir, e)))?;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
                        if matcher.is_match(fname) {
                            result.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        Ok(result)
    }
    pub(crate) fn new(batch_size: usize, file_paths: Vec<String>) -> Self {
        let batch_size = batch_size.clamp(32, 256);
        Self {
            batch_size,
            file_paths,
        }
    }

    pub(crate) async fn get_batch_log_from_file(
        &self,
        file_path: &str,
        offset: &mut usize,
        log_num: usize,
    ) -> Result<Vec<Vec<String>>> {
        let file = fs::File::open(file_path)
            .with_context(|| format!("Failed to open file: {}", file_path))?;
        let mut reader = io::BufReader::with_capacity(64 * 1024, file);
        let mut logs = Vec::with_capacity(log_num);
        let mut current_log = Vec::with_capacity(32);
        let mut batch_count = 0;
        while batch_count < log_num {
            loop {
                let mut buffer = String::with_capacity(512);
                let bytes_read = reader.read_line(&mut buffer)?;
                if bytes_read == 0 {
                    if !current_log.is_empty() {
                        logs.push(current_log);
                        current_log = Vec::with_capacity(32);
                    }
                    break;
                }
                let line = buffer.trim_end().to_string();
                if line.is_empty() {
                    continue;
                }
                current_log.push(line.clone());
                if line.ends_with(";") && !line.starts_with("use") && !line.starts_with("# ") {
                    break;
                }
            }
            if !current_log.is_empty() {
                logs.push(current_log);
                current_log = Vec::with_capacity(32);
                *offset += 1;
                batch_count += 1;
            }
        }
        tracing::debug!(target: "tihc.slowlog", "get_batch_log_from_file file={} batch_num={} logs_len={}", file_path, batch_count, logs.len());
        if !logs.is_empty() {
            for (i, log) in logs.iter().enumerate().take(2) {
                tracing::debug!(target: "tihc.slowlog", "batch[{}] lines: {}", i, log.len());
                for (j, line) in log.iter().enumerate().take(3) {
                    tracing::debug!(target: "tihc.slowlog", "batch[{}][{}]: {}", i, j, line);
                }
            }
        }
        Ok(logs)
    }
}

impl SlowLogService for SlowLogServiceImpl {
    fn parse_and_import<'a>(
        &'a self,
        log_dir: &'a str,
        pattern: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
        Box::pin(async move {
            // 1. 扫描文件
            let files = Self::scan_slowlog_files(log_dir, pattern)?;
            if files.is_empty() {
                tracing::warn!("No slowlog files matched");
                return Ok(());
            }
            // 2. 逐文件批量解析
            for file_path in files {
                let mut offset = 0;
                let mut batch_idx = 0;
                loop {
                    let batch = self
                        .get_batch_log_from_file(&file_path, &mut offset, self.batch_size)
                        .await?;
                    if batch.is_empty() {
                        break;
                    }
                    let parsed_result = crate::infrastructure::retriever::parse_log(&batch)
                        .map_err(anyhow::Error::from);
                    match &parsed_result {
                        Ok(rows) => tracing::info!(
                            "Parsed file={} batch={} rows={}",
                            file_path,
                            batch_idx,
                            rows.len()
                        ),
                        Err(e) => tracing::error!("Parse error: {e}"),
                    }
                    batch_idx += 1;
                }
            }
            Ok(())
        })
    }
    fn scan_files<'a>(
        &'a self,
        dir: &'a str,
        pattern: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<String>>> + Send + 'a>>
    {
        Box::pin(async move { Self::scan_slowlog_files(dir, pattern) })
    }

    fn parse_slow_log(
        self: Box<Self>,
        sender: tokio::sync::mpsc::Sender<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(async move {
            use std::path::Path;
            let file_paths = self.file_paths.clone();
            if file_paths.is_empty() {
                return Ok(());
            }
            for file_path in &file_paths {
                let path = Path::new(file_path);
                // Only process files that exist and are regular files
                if !path.exists() || !path.is_file() {
                    tracing::warn!("File not found or not a regular file: {}", file_path);
                    continue;
                }
                let mut offset = 0;
                let mut batch_idx = 0;
                loop {
                    let batch = self
                        .get_batch_log_from_file(file_path, &mut offset, self.batch_size)
                        .await?;
                    tracing::debug!(target: "tihc.slowlog", "parse_slow_log file={} batch_idx={} batch_len={}", file_path, batch_idx, batch.len());
                    if batch.is_empty() {
                        break;
                    }
                    let parsed_result = parse_log(&batch).map_err(anyhow::Error::from);
                    match &parsed_result {
                        Ok(rows) => {
                            tracing::debug!(target: "tihc.slowlog", "parse_log rows: {}", rows.len())
                        }
                        Err(e) => tracing::warn!(target: "tihc.slowlog", "parse_log error: {}", e),
                    }
                    sender.send(parsed_result).await?;
                    batch_idx += 1;
                }
            }
            Ok(())
        })
    }

    fn data_for_slow_log(
        &self,
        receiver: tokio::sync::mpsc::Receiver<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<SlowQueryRow>>> + Send>> {
        let s = stream::unfold(receiver, |mut receiver| async move {
            match receiver.recv().await {
                Some(Ok(rows)) if !rows.is_empty() => Some((Ok(rows), receiver)),
                Some(Err(e)) => Some((Err(e), receiver)),
                Some(Ok(_)) => None,
                None => None,
            }
        });
        Box::pin(s) as Pin<Box<dyn Stream<Item = Result<Vec<SlowQueryRow>>> + Send>>
    }
}
