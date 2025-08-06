use crate::infrastructure::mysql_writer::{get_mysql_pool, init_db_and_table};
use anyhow::{Context, Result};
use std::io::BufRead;
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
}

use crate::domain::connection::Connection;

pub(crate) struct SlowLogServiceImpl {
    batch_size: usize,
    conn: Connection,
}

impl SlowLogServiceImpl {
    /// 扫描 slowlog 文件列表（仅供 web 前端调用，CLI 不应使用）
    pub(crate) fn scan_slowlog_files(dir: &str, pattern: &str) -> Result<Vec<String>> {
        use regex::Regex;
        let mut result = Vec::new();
        let re = Regex::new(pattern)
            .map_err(|e| anyhow::anyhow!(format!("Invalid regex pattern: {}: {}", pattern, e)))?;
        let entries = std::fs::read_dir(dir)
            .map_err(|e| anyhow::anyhow!(format!("Failed to read dir {}: {}", dir, e)))?;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
                        if re.is_match(fname) {
                            result.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        Ok(result)
    }
    pub(crate) fn new(batch_size: usize, conn: Connection) -> Self {
        let batch_size = batch_size.clamp(32, 256);
        Self { batch_size, conn }
    }

    pub(crate) fn get_batch_log_from_reader(
        &self,
        reader: &mut io::BufReader<fs::File>,
        log_num: usize,
    ) -> Result<Vec<Vec<String>>> {
        let mut logs = Vec::with_capacity(log_num);
        let mut batch_count = 0;
        let mut eof = false;
        while batch_count < log_num && !eof {
            let mut current_log = Vec::with_capacity(32);
            loop {
                let mut buffer = String::with_capacity(512);
                let bytes_read = reader.read_line(&mut buffer)?;
                if bytes_read == 0 {
                    // 文件读完，处理最后一批数据
                    eof = true;
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
                batch_count += 1;
            }
        }
        // 如果已经到达文件结尾且没有读到任何日志，则返回空 batch，触发外层 break
        if logs.is_empty() {
            return Ok(Vec::new());
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
                tracing::warn!(target: "slowlog_api", "[parse_and_import] No slowlog files found, dir={}", log_dir);
                return Ok(());
            }
            // 0. 只支持 TiDB
            if self.conn.engine.to_lowercase() != "tidb" {
                tracing::error!(target: "slowlog_api", "[parse_and_import] Only TiDB is supported, found engine={}", self.conn.engine);
                return Err(anyhow::anyhow!("Only TiDB is supported"));
            }
            // 2. 初始化数据库连接和表（由 infra 层统一管理）
            let pool = get_mysql_pool(&self.conn).await?;
            init_db_and_table(&pool).await?;
            // 3. 跨文件批量处理，使用 get_batch_log_from_file，保证每批数据真实且不补全
            for file_path in &files {
                tracing::info!(target: "slowlog_api", "[parse_and_import] Processing file: {}", file_path);
                let file = fs::File::open(file_path)
                    .with_context(|| format!("Failed to open file: {}", file_path))?;
                let mut reader = io::BufReader::with_capacity(64 * 1024, file);
                let mut batch_idx = 0;
                let mut total_rows_processed = 0;
                loop {
                    let batch = self.get_batch_log_from_reader(&mut reader, self.batch_size)?;
                    tracing::debug!(target: "slowlog_api", "[parse_and_import] batch_idx={}, batch_size={}", batch_idx, batch.len());
                    if batch.is_empty() {
                        tracing::info!(target: "slowlog_api", "[parse_and_import] File {} completed: {} batches, {} total rows processed", file_path, batch_idx, total_rows_processed);
                        break;
                    }
                    batch_idx += 1;
                    let parsed_result = crate::infrastructure::retriever::parse_log(&batch)
                        .map_err(anyhow::Error::from);
                    match &parsed_result {
                        Ok(rows) => {
                            if rows.is_empty() {
                                tracing::warn!(target: "slowlog_api", "[parse_and_import] Batch {} in file {} parsed 0 rows", batch_idx, file_path);
                            }
                            if let Err(e) =
                                crate::infrastructure::mysql_writer::write_slowlog_rows(rows, &pool)
                                    .await
                            {
                                tracing::error!(target: "slowlog_api", "[parse_and_import] Failed to write rows to MySQL: {:?}", e);
                            } else {
                                total_rows_processed += rows.len();
                                tracing::debug!(target: "slowlog_api", "[parse_and_import] Batch {} processed: {} rows", batch_idx, rows.len());
                                
                                // Dynamic progress reporting frequency based on file size
                                let report_frequency = if batch_idx > 1000 { 500 } else { 100 };
                                if batch_idx % report_frequency == 0 {
                                    tracing::info!(target: "slowlog_api", "[parse_and_import] Progress: {} batches processed, {} total rows for file {}", batch_idx, total_rows_processed, file_path);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!(target: "slowlog_api", "[parse_and_import] Failed to parse batch {} in file {}: {:?}", batch_idx, file_path, e);
                        }
                    }
                }
            }
            tracing::info!(target: "slowlog_api", "[parse_and_import] All {} files processed successfully.", files.len());
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
}
