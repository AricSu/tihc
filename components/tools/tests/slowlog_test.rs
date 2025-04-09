use futures_util::StreamExt;
use std::io::{self, Write};
use tempfile::NamedTempFile;
use tools::{slow_log_retriever::split_by_colon, slow_query::SlowQueryRow};

// Test data constants
const TEST_LOG_PATH: &str = "/Users/aric/Downloads/tidb_slow_query-2025-03/tidb_slow_query.log";
const BATCH_SIZE: usize = 64;
const CHANNEL_BUFFER_SIZE: usize = 1024;

#[test]
fn test_split_by_colon() {
    let cases = vec![
        ("", vec![], vec![]),
        ("123a", vec!["123a"], vec![""]),
        ("1a: 2b", vec!["1a"], vec!["2b"]),
        // ... existing test cases ...
    ];

    for (line, expected_fields, expected_values) in cases {
        let (actual_fields, actual_values) = split_by_colon(line);
        assert_eq!(actual_fields, expected_fields);
        assert_eq!(actual_values, expected_values);
    }
}

#[test]
fn test_retriever_parse_log() {
    let slow_log = r#"# Time: 2019-04-28T15:24:04.309074+08:00
# Txn_start_ts: 405888132465033227
# User@Host: root[root] @ localhost [127.0.0.1]
# Session_alias: alias123
# Exec_retry_time: 0.12 Exec_retry_count: 57
# Query_time: 0.216905
# Cop_time: 0.38 Process_time: 0.021 Request_count: 1 Total_keys: 637 Processed_keys: 436
# Rocksdb_delete_skipped_count: 10 Rocksdb_key_skipped_count: 10 Rocksdb_block_cache_hit_count: 10 Rocksdb_block_read_count: 10 Rocksdb_block_read_byte: 100
# Is_internal: true
# Digest: 42a1c8aae6f133e934d4bf0147491709a8812ea05ff8819ec522780fe657b772
# Stats: t1:1,t2:2
# Cop_proc_avg: 0.1 Cop_proc_p90: 0.2 Cop_proc_max: 0.03 Cop_proc_addr: 127.0.0.1:20160
# Cop_wait_avg: 0.05 Cop_wait_p90: 0.6 Cop_wait_max: 0.8 Cop_wait_addr: 0.0.0.0:20160
# Cop_backoff_regionMiss_total_times: 200 Cop_backoff_regionMiss_total_time: 0.2 Cop_backoff_regionMiss_max_time: 0.2 Cop_backoff_regionMiss_max_addr: 127.0.0.1 Cop_backoff_regionMiss_avg_time: 0.2 Cop_backoff_regionMiss_p90_time: 0.2
# Cop_backoff_rpcPD_total_times: 200 Cop_backoff_rpcPD_total_time: 0.2 Cop_backoff_rpcPD_max_time: 0.2 Cop_backoff_rpcPD_max_addr: 127.0.0.1 Cop_backoff_rpcPD_avg_time: 0.2 Cop_backoff_rpcPD_p90_time: 0.2
# Cop_backoff_rpcTiKV_total_times: 200 Cop_backoff_rpcTiKV_total_time: 0.2 Cop_backoff_rpcTiKV_max_time: 0.2 Cop_backoff_rpcTiKV_max_addr: 127.0.0.1 Cop_backoff_rpcTiKV_avg_time: 0.2 Cop_backoff_rpcTiKV_p90_time: 0.2
# Mem_max: 70724
# Disk_max: 65536
# Plan_from_cache: true
# Plan_from_binding: true
# Succ: false
# IsExplicitTxn: true
# Resource_group: default
# Request_unit_read: 2.158
# Request_unit_write: 2.123
# Time_queued_by_rc: 0.05
# Tidb_cpu_time: 0.01
# Tikv_cpu_time: 0.021
# Plan_digest: 60e9378c746d9a2be1c791047e008967cf252eb6de9167ad3aa6098fa2d523f4
# Prev_stmt: update t set i = 1;
use test;
select * from t;"#;

    let expected = r#"{"time":"2019-04-28T15:24:04.309074+08:00","txn_start_ts":405888132465033227,"user":"root","host":"localhost","conn_id":0,"session_alias":"alias123","exec_retry_count":57,"exec_retry_time":0.12,"query_time":0.216905,"parse_time":0.0,"compile_time":0.0,"rewrite_time":0.0,"preproc_subqueries":0,"preproc_subqueries_time":0.0,"optimize_time":0.0,"wait_ts":0.0,"prewrite_time":0.0,"wait_prewrite_binlog_time":0.0,"commit_time":0.0,"get_commit_ts_time":0.0,"commit_backoff_time":0.0,"backoff_types":"","resolve_lock_time":0.0,"local_latch_wait_time":0.0,"write_keys":0,"write_size":0,"prewrite_region":0,"txn_retry":0,"cop_time":0.38,"process_time":0.021,"wait_time":0.0,"backoff_time":0.0,"lock_keys_time":0.0,"request_count":1,"total_keys":637,"process_keys":0,"rocksdb_delete_skipped_count":10,"rocksdb_key_skipped_count":10,"rocksdb_block_cache_hit_count":10,"rocksdb_block_read_count":10,"rocksdb_block_read_byte":100,"db":"","index_names":"","is_internal":true,"digest":"42a1c8aae6f133e934d4bf0147491709a8812ea05ff8819ec522780fe657b772","stats":"t1:1,t2:2","cop_proc_avg":0.1,"cop_proc_p90":0.2,"cop_proc_max":0.03,"cop_proc_addr":"127.0.0.1:20160","cop_wait_avg":0.05,"cop_wait_p90":0.6,"cop_wait_max":0.8,"cop_wait_addr":"0.0.0.0:20160","mem_max":70724,"disk_max":65536,"kv_total":0.0,"pd_total":0.0,"backoff_total":0.0,"write_sql_response_total":0.0,"result_rows":0,"warnings":"","backoff_detail":"Cop_backoff_regionMiss_total_times: 200 Cop_backoff_regionMiss_total_time: 0.2 Cop_backoff_regionMiss_max_time: 0.2 Cop_backoff_regionMiss_max_addr: 127.0.0.1 Cop_backoff_regionMiss_avg_time: 0.2 Cop_backoff_regionMiss_p90_time: 0.2 Cop_backoff_rpcPD_total_times: 200 Cop_backoff_rpcPD_total_time: 0.2 Cop_backoff_rpcPD_max_time: 0.2 Cop_backoff_rpcPD_max_addr: 127.0.0.1 Cop_backoff_rpcPD_avg_time: 0.2 Cop_backoff_rpcPD_p90_time: 0.2 Cop_backoff_rpcTiKV_total_times: 200 Cop_backoff_rpcTiKV_total_time: 0.2 Cop_backoff_rpcTiKV_max_time: 0.2 Cop_backoff_rpcTiKV_max_addr: 127.0.0.1 Cop_backoff_rpcTiKV_avg_time: 0.2 Cop_backoff_rpcTiKV_p90_time: 0.2","prepared":false,"succ":false,"is_explicit_txn":true,"is_write_cache_table":false,"plan_from_cache":true,"plan_from_binding":true,"has_more_results":false,"resource_group":"default","request_unit_read":2.158,"request_unit_write":2.123,"time_queued_by_rc":0.05,"tidb_cpu_time":0.01,"tikv_cpu_time":0.021,"plan":"","plan_digest":"60e9378c746d9a2be1c791047e008967cf252eb6de9167ad3aa6098fa2d523f4","binary_plan":"","prev_stmt":"update t set i = 1;","query":"select * from t;"}"#;

    let input_log = slow_log
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let result = tools::slow_log_retriever::parse_log(&[input_log]).unwrap();
    let actual = serde_json::to_string(&result[0]).unwrap();
    assert_eq!(actual, expected);
}

#[tokio::test]
async fn test_get_batch_log_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![(
        "single_query",
        r#"# Time: 2023-01-01T00:00:00.000000+08:00
# Query_time: 0.1
SELECT 1;"#,
        1,
    )];

    for (case_name, input, expected) in test_cases.iter() {
        let temp_file = create_temp_file(input).map_err(|e| anyhow::anyhow!(e))?;
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let mut retriever =
            tools::slow_log_retriever::SlowQueryRetriever::new(BATCH_SIZE, vec![file_path.clone()]);

        let mut reader =
            io::BufReader::new(std::fs::File::open(&file_path).map_err(|e| anyhow::anyhow!(e))?);
        let mut offset = 0;
        let logs = retriever
            .get_batch_log(&mut reader, &mut offset, *expected)
            .await?;

        assert_eq!(
            logs.len(),
            *expected,
            "Test case '{}' failed: expected {} logs, got {}",
            case_name,
            expected,
            logs.len()
        );
    }
    Ok(())
}

#[tokio::test]
async fn test_slow_query_retriever() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    let mut retriever = tools::slow_log_retriever::SlowQueryRetriever::new(
        BATCH_SIZE,
        vec![TEST_LOG_PATH.to_string()],
    );

    let (sender, receiver) = tokio::sync::mpsc::channel(CHANNEL_BUFFER_SIZE);
    retriever.parse_slow_log(sender).await?;

    let stream = retriever.data_for_slow_log(receiver).await;
    let stream = Box::pin(stream);
    let count = process_stream(stream).await?;

    assert!(count > 0, "No slow query logs were parsed");
    tracing::info!("Successfully parsed {} slow query logs", count);

    Ok(())
}

// Helper functions
fn setup_logging() {
    let _ = tracing_subscriber::fmt().try_init();
}

fn create_temp_file(content: &str) -> io::Result<NamedTempFile> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(content.as_bytes())?;
    temp_file.flush()?;
    Ok(temp_file)
}

async fn process_stream(
    stream: impl futures::Stream<Item = Result<Vec<SlowQueryRow>, anyhow::Error>> + Unpin,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count = 0;
    let mut stream = Box::pin(stream);

    while let Some(Ok(rows)) = stream.next().await {
        count += rows.len();
    }

    Ok(count)
}
