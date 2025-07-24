use anyhow::{Context, Result};
use crate::domain::table::SlowQueryRow;
use crate::infrastructure::retriever::parse_log;
use std::{fs, io};
use std::io::BufRead;
use futures::{Stream, stream};
use std::future::Future;
use std::pin::Pin;

pub trait SlowLogService: Send + Sync {
    fn parse_slow_log(
        self: Box<Self>,
        sender: tokio::sync::mpsc::Sender<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn data_for_slow_log(
        &self,
        receiver: tokio::sync::mpsc::Receiver<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<SlowQueryRow>>> + Send>>;
}

pub struct SlowLogServiceImpl {
    pub batch_size: usize,
    pub file_paths: Vec<String>,
}

impl SlowLogServiceImpl {
    pub fn new(batch_size: usize, file_paths: Vec<String>) -> Self {
        let batch_size = batch_size.clamp(32, 256);
        Self {
            batch_size,
            file_paths,
        }
    }

    /// 读取指定文件的日志批次
    async fn get_batch_log_from_file(
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
                    }
                    return Ok(logs);
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
        Ok(logs)
    }
}

impl SlowLogService for SlowLogServiceImpl {
    fn parse_slow_log(
        self: Box<Self>,
        sender: tokio::sync::mpsc::Sender<Result<Vec<SlowQueryRow>>>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(async move {
            if self.file_paths.is_empty() {
                return Ok(());
            }
            for file_path in &self.file_paths {
                let mut offset = 0;
                loop {
                    let batch = self.get_batch_log_from_file(file_path, &mut offset, self.batch_size).await?;
                    if batch.is_empty() {
                        break;
                    }
                    let parsed_result = parse_log(&batch).map_err(anyhow::Error::from);
                    sender.send(parsed_result).await?;
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
