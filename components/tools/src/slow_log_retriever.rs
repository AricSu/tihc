use crate::{slow_query::SlowQueryRow, slowlog_fields::SlowLogFields};
use chrono::DateTime;
use std::io::{self, BufRead};
use futures::stream::Stream;

pub fn split_by_colon(line: &str) -> (Vec<String>, Vec<String>) {
    fn is_letter_or_numeric(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn find_matched_right_bracket(line: &str, left_bracket_idx: usize) -> Option<usize> {
        let chars: Vec<char> = line.chars().collect();
        let left_bracket = chars[left_bracket_idx];
        let right_bracket = match left_bracket {
            '[' => ']',
            '{' => '}',
            _ => return None,
        };

        let mut current = left_bracket_idx;
        let mut left_bracket_cnt = 0;

        while current < chars.len() {
            let c = chars[current];
            if c == left_bracket {
                left_bracket_cnt += 1;
                current += 1;
            } else if c == right_bracket {
                left_bracket_cnt -= 1;
                if left_bracket_cnt > 0 {
                    current += 1;
                } else if left_bracket_cnt == 0 {
                    if current + 1 < chars.len() && !chars[current + 1].is_whitespace() {
                        return None;
                    }
                    return Some(current);
                } else {
                    return None;
                }
            } else {
                current += 1;
            }
        }
        None
    }

    let mut fields = Vec::new();
    let mut values = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut current = 0;
    let mut parse_key = true;
    let mut err_msg = None;

    while current < chars.len() {
        if parse_key {
            // 查找键的开始位置
            while current < chars.len() && !is_letter_or_numeric(chars[current]) {
                current += 1;
            }
            let start = current;
            if current >= chars.len() {
                break;
            }

            // 查找冒号位置
            while current < chars.len() && chars[current] != ':' {
                current += 1;
            }
            fields.push(line[start..current].trim().to_string());

            // 跳过冒号和空格
            current += 2;
            if current >= chars.len() {
                values.push(String::new());
            }
            parse_key = false;
        } else {
            let start = current;
            if current < chars.len() && (chars[current] == '{' || chars[current] == '[') {
                if let Some(r_brace_idx) = find_matched_right_bracket(line, current) {
                    current = r_brace_idx + 1;
                } else {
                    err_msg = Some("Braces matched error");
                    break;
                }
            } else {
                while current < chars.len() && !chars[current].is_whitespace() {
                    current += 1;
                }
                // 处理空值情况: "Key: Key:"
                if current > 0 && chars[current - 1] == ':' {
                    values.push(String::new());
                    current = start;
                    parse_key = true;
                    continue;
                }
            }
            values.push(line[start..current].trim().to_string());
            parse_key = true;
        }
    }

    if let Some(msg) = err_msg {
        tracing::warn!("slow query parse slow log error: {}, Log: {}", msg, line);
        return (Vec::new(), Vec::new());
    }

    if fields.len() != values.len() {
        tracing::warn!(
            "slow query parse slow log error: field_count={}, value_count={}, Log: {}",
            fields.len(),
            values.len(),
            line
        );
        return (Vec::new(), Vec::new());
    }

    (fields, values)
}

fn parse_user_or_host_value(value: &str) -> String {
    // 处理格式如: root[root] 或 localhost [127.0.0.1]
    value.split('[').next().unwrap_or("").trim().to_string()
}

pub fn parse_time(time_str: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        return dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
    }
    // 如果解析失败，记录警告并返回原始字符串
    tracing::warn!("Failed to parse time string: {}", time_str);
    time_str.to_string()
}

/// 新增通用字段设置方法
fn set_column_value(query: &mut SlowQueryRow, field: &str, value: &str, line_num: usize) {
    // 定义一个内部函数来处理解析错误并记录日志
    fn parse_with_log<T: std::str::FromStr + std::default::Default>(
        value: &str,
        field: &str,
        line_num: usize,
    ) -> T
    where
        T::Err: std::fmt::Display,
    {
        match value.parse::<T>() {
            Ok(val) => val,
            Err(e) => {
                tracing::warn!(
                    "解析慢查询日志字段失败: field={}, value={}, line={}, error={}",
                    field,
                    value,
                    line_num,
                    e
                );
                T::default()
            }
        }
    }

    match field {
        // 整数类型
        SlowLogFields::EXEC_RETRY_COUNT => {
            query.exec_retry_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::PREPROC_SUBQUERIES => {
            query.preproc_subqueries = parse_with_log(value, field, line_num)
        }
        SlowLogFields::REQUEST_COUNT => {
            query.request_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::TOTAL_KEYS => query.total_keys = parse_with_log(value, field, line_num),
        SlowLogFields::PROCESS_KEYS => query.process_keys = parse_with_log(value, field, line_num),
        SlowLogFields::ROCKSDB_DELETE_SKIPPED_COUNT => {
            query.rocksdb_delete_skipped_count = parse_with_log(value, field, line_num)
        }
        SlowLogFields::ROCKSDB_KEY_SKIPPED_COUNT => {
            query.rocksdb_key_skipped_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_CACHE_HIT_COUNT => {
            query.rocksdb_block_cache_hit_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_READ_COUNT => {
            query.rocksdb_block_read_count = value.parse().unwrap_or_default()
        }
        SlowLogFields::ROCKSDB_BLOCK_READ_BYTE => {
            query.rocksdb_block_read_byte = value.parse().unwrap_or_default()
        }
        SlowLogFields::TXN_RETRY => query.txn_retry = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_KEYS => query.write_keys = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_SIZE => query.write_size = value.parse().unwrap_or_default(),
        SlowLogFields::PREWRITE_REGION => query.prewrite_region = value.parse().unwrap_or_default(),
        SlowLogFields::RESULT_ROWS => query.result_rows = value.parse().unwrap_or_default(),
        SlowLogFields::MEM_MAX => query.mem_max = value.parse().unwrap_or_default(),
        SlowLogFields::DISK_MAX => query.disk_max = value.parse().unwrap_or_default(),
        SlowLogFields::CONN_ID => query.conn_id = value.parse().unwrap_or_default(),
        SlowLogFields::TXN_START_TS => query.txn_start_ts = value.parse().unwrap_or_default(),

        // 浮点数类型
        SlowLogFields::QUERY_TIME => query.query_time = value.parse().unwrap_or_default(),
        SlowLogFields::EXEC_RETRY_TIME => query.exec_retry_time = value.parse().unwrap_or_default(),
        SlowLogFields::PARSE_TIME => query.parse_time = value.parse().unwrap_or_default(),
        SlowLogFields::COMPILE_TIME => query.compile_time = value.parse().unwrap_or_default(),
        SlowLogFields::REWRITE_TIME => query.rewrite_time = value.parse().unwrap_or_default(),
        SlowLogFields::PREPROC_SUBQUERIES_TIME => {
            query.preproc_subqueries_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::OPTIMIZE_TIME => query.optimize_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_TS_TIME => query.wait_ts = value.parse().unwrap_or_default(),
        SlowLogFields::PREWRITE_TIME => query.prewrite_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_PREWRITE_BINLOG_TIME => {
            query.wait_prewrite_binlog_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COMMIT_TIME => query.commit_time = value.parse().unwrap_or_default(),
        SlowLogFields::GET_COMMIT_TS_TIME => {
            query.get_commit_ts_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COMMIT_BACKOFF_TIME => {
            query.commit_backoff_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::RESOLVE_LOCK_TIME => {
            query.resolve_lock_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::LOCAL_LATCH_WAIT_TIME => {
            query.local_latch_wait_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::COP_TIME => query.cop_time = value.parse().unwrap_or_default(),
        SlowLogFields::PROCESS_TIME => query.process_time = value.parse().unwrap_or_default(),
        SlowLogFields::WAIT_TIME => query.wait_time = value.parse().unwrap_or_default(),
        SlowLogFields::BACKOFF_TIME => query.backoff_time = value.parse().unwrap_or_default(),
        SlowLogFields::LOCK_KEYS_TIME => query.lock_keys_time = value.parse().unwrap_or_default(),
        SlowLogFields::WRITE_SQL_RESPONSE_TOTAL => {
            query.write_sql_response_total = value.parse().unwrap_or_default()
        }
        SlowLogFields::REQUEST_UNIT_READ => {
            query.request_unit_read = value.parse().unwrap_or_default()
        }
        SlowLogFields::REQUEST_UNIT_WRITE => {
            query.request_unit_write = value.parse().unwrap_or_default()
        }
        SlowLogFields::TIME_QUEUED_BY_RC => {
            query.time_queued_by_rc = value.parse().unwrap_or_default()
        }
        SlowLogFields::KV_TOTAL => query.kv_total = value.parse().unwrap_or_default(),
        SlowLogFields::PD_TOTAL => query.pd_total = value.parse().unwrap_or_default(),
        SlowLogFields::BACKOFF_TOTAL => query.backoff_total = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_AVG => query.cop_proc_avg = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_P90 => query.cop_proc_p90 = value.parse().unwrap_or_default(),
        SlowLogFields::COP_PROC_MAX => query.cop_proc_max = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_AVG => query.cop_wait_avg = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_P90 => query.cop_wait_p90 = value.parse().unwrap_or_default(),
        SlowLogFields::COP_WAIT_MAX => query.cop_wait_max = value.parse().unwrap_or_default(),
        SlowLogFields::TIDB_CPU_USAGE_DURATION => {
            query.tidb_cpu_time = value.parse().unwrap_or_default()
        }
        SlowLogFields::TIKV_CPU_USAGE_DURATION => {
            query.tikv_cpu_time = value.parse().unwrap_or_default()
        }

        // 布尔类型
        SlowLogFields::IS_INTERNAL => query.is_internal = value == "true",
        SlowLogFields::PREPARED => query.prepared = value == "true",
        SlowLogFields::SUCC => query.succ = value == "true",
        SlowLogFields::IS_EXPLICIT_TXN => query.is_explicit_txn = value == "true",
        SlowLogFields::IS_WRITE_CACHE_TABLE => query.is_write_cache_table = value == "true",
        SlowLogFields::PLAN_FROM_CACHE => query.plan_from_cache = value == "true",
        SlowLogFields::PLAN_FROM_BINDING => query.plan_from_binding = value == "true",
        SlowLogFields::HAS_MORE_RESULTS => query.has_more_results = value == "true",

        // 字符串类型
        SlowLogFields::DIGEST => query.digest = value.to_string(),
        SlowLogFields::STATS_INFO => query.stats = value.to_string(),
        SlowLogFields::INDEX_NAMES => query.index_names = value.to_string(),
        SlowLogFields::QUERY => query.query = value.to_string(),
        SlowLogFields::PLAN => query.plan = value.to_string(),
        SlowLogFields::PLAN_DIGEST => query.plan_digest = value.to_string(),
        SlowLogFields::PREV_STMT => query.prev_stmt = value.to_string(),
        SlowLogFields::USER => query.user = value.to_string(),
        SlowLogFields::HOST => query.host = value.to_string(),
        SlowLogFields::SESSION_ALIAS => query.session_alias = value.to_string(),
        SlowLogFields::DB => query.db = value.to_string(),
        SlowLogFields::RESOURCE_GROUP => query.resource_group = value.to_string(),
        SlowLogFields::BACKOFF_TYPES => query.backoff_types = value.to_string(),
        SlowLogFields::COP_PROC_ADDR => query.cop_proc_addr = value.to_string(),
        SlowLogFields::COP_WAIT_ADDR => query.cop_wait_addr = value.to_string(),
        SlowLogFields::BINARY_PLAN => query.binary_plan = value.to_string(),
        SlowLogFields::BACKOFF_DETAIL => query.backoff_detail = value.to_string(),

        // 其他特殊处理
        _ => {}
    }
}

pub fn parse_log(logs: &[Vec<String>]) -> io::Result<Vec<SlowQueryRow>> {
    let mut data = Vec::new();

    for log in logs {
        let mut query = SlowQueryRow::default();
        let mut start_flag = false;

        for (index, line) in log.iter().enumerate() {
            let line_num = index + 1;

            // 解析开始标记行
            if line.starts_with(SlowLogFields::START_PREFIX) {
                query = SlowQueryRow::default();
                // query.time = parse_time(&line[SlowLogFields::START_PREFIX.len()..].trim());
                query.time = line[SlowLogFields::START_PREFIX.len()..].trim().to_string();
                start_flag = true;
                continue;
            }

            if !start_flag {
                continue;
            }

            // 处理 ROW_PREFIX 开头的行
            if line.starts_with(SlowLogFields::ROW_PREFIX) {
                let content = &line[SlowLogFields::ROW_PREFIX.len()..];

                // 处理特殊字段分支
                if content.starts_with(SlowLogFields::PREV_STMT_PREFIX) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::PREV_STMT,
                        &content[SlowLogFields::PREV_STMT_PREFIX.len()..],
                        line_num,
                    );
                } else if content.starts_with(SlowLogFields::USER_AND_HOST) {
                    let value = content
                        [SlowLogFields::USER_AND_HOST.len() + SlowLogFields::SPACE_MARK.len()..]
                        .trim();
                    if let Some((user_val, host)) = value.split_once('@') {
                        let user = parse_user_or_host_value(user_val);
                        set_column_value(&mut query, SlowLogFields::USER, &user, line_num);
                        let host = parse_user_or_host_value(host);
                        set_column_value(&mut query, SlowLogFields::HOST, &host, line_num);
                    }
                } else if content.starts_with(SlowLogFields::COP_BACKOFF_PREFIX) {
                    let new_detail = content.to_string();
                    if !query.backoff_detail.is_empty() {
                        query.backoff_detail.push_str(" ");
                        query.backoff_detail.push_str(&new_detail);
                    } else {
                        query.backoff_detail = new_detail;
                    }
                } else if content.starts_with(SlowLogFields::WARNINGS) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::WARNINGS,
                        &content[SlowLogFields::WARNINGS.len() + SlowLogFields::SPACE_MARK.len()..],
                        line_num,
                    );
                } else if content.starts_with(SlowLogFields::DB) {
                    set_column_value(
                        &mut query,
                        SlowLogFields::DB,
                        &content[SlowLogFields::DB.len() + SlowLogFields::SPACE_MARK.len()..],
                        line_num,
                    );
                } else {
                    // 通用字段解析
                    let (fields, values) = split_by_colon(content);
                    for (field, value) in fields.iter().zip(values.iter()) {
                        set_column_value(&mut query, field, value, line_num);
                    }
                }
            }
            // 处理 SQL 结束标记
            else if line.ends_with(SlowLogFields::SQL_SUFFIX) {
                // 忽略 use 语句
                if line.starts_with("use") {
                    continue;
                }

                // 设置最终查询语句
                set_column_value(&mut query, SlowLogFields::QUERY, line, line_num);

                // 添加完整查询到结果集
                data.push(query);

                // 重置状态
                query = SlowQueryRow::default();
                start_flag = false;
            } else {
                start_flag = false;
            }
        }
    }

    Ok(data)
}

/// Main structure for retrieving and processing slow query logs
#[derive(Debug)]
pub struct SlowQueryRetriever {
    /// Number of logs to process in each batch
    pub batch_size: usize,
    /// Index of the current file being processed
    pub current_file_index: usize,
    /// List of file paths to process
    pub file_paths: Vec<String>,
    /// Current line number in the file
    pub file_line: usize,
}

impl SlowQueryRetriever {
    // 简化 new 方法
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size: batch_size.clamp(32, 256),
            current_file_index: 0,
            file_paths: Vec::new(),
            file_line: 0,
        }
    }
}

impl SlowQueryRetriever {
    // 简化 get_next_reader 方法
    async fn get_next_reader(&mut self) -> io::Result<Option<io::BufReader<std::fs::File>>> {
        self.current_file_index += 1;
        if self.current_file_index < self.file_paths.len() {
            let file = std::fs::File::open(&self.file_paths[self.current_file_index])?;
            Ok(Some(io::BufReader::new(file)))
        } else {
            Ok(None)
        }
    }

    // 简化 get_batch_log 方法
    pub async fn get_batch_log(
        &mut self,
        reader: &mut io::BufReader<std::fs::File>,
        offset: &mut usize,
        log_num: usize,
    ) -> io::Result<Vec<Vec<String>>> {
        let mut logs = Vec::with_capacity(log_num);
        let mut current_log = Vec::new();

        for _ in 0..log_num {
            loop {
                self.file_line += 1;
                let mut buffer = String::new();
                if reader.read_line(&mut buffer)? == 0 {
                    self.file_line = 0;
                    if let Some(new_reader) = self.get_next_reader().await? {
                        *reader = new_reader;
                        continue;
                    }
                    if !current_log.is_empty() {
                        logs.push(current_log);
                    }
                    return Ok(logs);
                }

                let line = buffer.trim_end().to_string();
                current_log.push(line.clone());

                if line.ends_with(SlowLogFields::SQL_SUFFIX) 
                    && !line.starts_with("use") 
                    && !line.starts_with(SlowLogFields::ROW_PREFIX) {
                    break;
                }
            }

            logs.push(current_log);
            current_log = Vec::new();
            *offset += 1;
        }

        Ok(logs)
    }



    // 简化 parse_slow_log 方法
    pub async fn parse_slow_log(
        &mut self,
        reader: &mut io::BufReader<std::fs::File>,
        sender: tokio::sync::mpsc::Sender<io::Result<Vec<SlowQueryRow>>>,
    ) -> io::Result<()> {
        let mut offset = 0;

        while let Ok(batch) = self.get_batch_log(reader, &mut offset, self.batch_size).await {
            if batch.is_empty() {
                break;
            }
            sender.send(parse_log(&batch)).await.map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to send result: {}", e))
            })?;
        }

        Ok(())
    }

    // 简化 data_for_slow_log 方法
    // 修改为返回异步流，解耦合数据库写入操作
    pub async fn data_for_slow_log(
        &mut self,
        receiver: tokio::sync::mpsc::Receiver<io::Result<Vec<SlowQueryRow>>>,
    ) -> impl futures::Stream<Item = io::Result<Vec<SlowQueryRow>>> + '_ {
        futures::stream::unfold(receiver, |mut receiver| async move {
            match receiver.recv().await {
                Some(Ok(rows)) if !rows.is_empty() => Some((Ok(rows), receiver)),
                Some(Err(e)) => Some((Err(e), receiver)),
                _ => None,
            }
        })
    }
}
