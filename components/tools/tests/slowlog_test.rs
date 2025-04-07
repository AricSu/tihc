use chrono::Offset;
use sqlx::Executor;
use std::io::{self, BufRead, Cursor};
use test;
use tools::{
    slow_log_retriever::{get_batch_log, parse_time, split_by_colon},
    slow_query::{DbOps, SlowQueryRow},
};

#[test]
fn test_split_by_colon() {
    let cases = vec![
        ("", vec![], vec![]),
        ("123a", vec!["123a"], vec![""]),
        ("1a: 2b", vec!["1a"], vec!["2b"]),
        (
            "1a: [2b 3c] 4d: 5e",
            vec!["1a", "4d"],
            vec!["[2b 3c]", "5e"],
        ),
        (
            "1a: [2b,3c] 4d: 5e",
            vec!["1a", "4d"],
            vec!["[2b,3c]", "5e"],
        ),
        (
            "1a: [2b,[3c: 3cc]] 4d: 5e",
            vec!["1a", "4d"],
            vec!["[2b,[3c: 3cc]]", "5e"],
        ),
        (
            "1a: {2b 3c} 4d: 5e",
            vec!["1a", "4d"],
            vec!["{2b 3c}", "5e"],
        ),
        (
            "1a: {2b,3c} 4d: 5e",
            vec!["1a", "4d"],
            vec!["{2b,3c}", "5e"],
        ),
        (
            "1a: {2b,{3c: 3cc}} 4d: 5e",
            vec!["1a", "4d"],
            vec!["{2b,{3c: 3cc}}", "5e"],
        ),
        ("1a: {{{2b,{3c: 3cc}} 4d: 5e", vec![], vec![]),
        ("1a: [2b,[3c: 3cc]]]] 4d: 5e", vec![], vec![]),
        (
            "Time: 2021-09-08T14:39:54.506967433+08:00",
            vec!["Time"],
            vec!["2021-09-08T14:39:54.506967433+08:00"],
        ),
        (
            "Cop_proc_avg: 0 Cop_proc_addr: Cop_proc_max: Cop_proc_min: ",
            vec![
                "Cop_proc_avg",
                "Cop_proc_addr",
                "Cop_proc_max",
                "Cop_proc_min",
            ],
            vec!["0", "", "", ""],
        ),
    ];

    for (line, expected_fields, expected_values) in cases {
        let (actual_fields, actual_values) = split_by_colon(line);
        assert_eq!(actual_fields, expected_fields);
        assert_eq!(actual_values, expected_values);
    }
}

#[test]
fn test_parse_time() {
    let test_cases = vec![
        (
            "2019-04-28T15:24:04.309074+08:00",
            "2019-04-28 15:24:04.309074",
        ),
        (
            "2022-04-21T14:44:54.103041447+08:00",
            "2022-04-21 14:44:54.103041", // 只保留 6 位小数
        ),
        (
            "2025-03-25T04:30:25.327651606Z",
            "2025-03-25 04:30:25.327651", // 只保留 6 位小数
        ),
        ("2025-03-25T04:30:25.32Z", "2025-03-25 04:30:25.320000"),
        ("2025-03-25T04:30:25Z", "2025-03-25 04:30:25"), // 不补全小数
        ("2025-03-25T04:30:25", "2025-03-25 04:30:25"),  // 不补全小数
        (
            "2025-03-25T04:30:25.123456789+08:00",
            "2025-03-25 04:30:25.123456", // 只保留 6 位小数
        ),
    ];

    for (input, expected) in test_cases {
        assert_eq!(
            parse_time(input), // 直接使用 parse_time 的返回值
            expected,
            "Failed to parse: {}",
            input
        );
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

    let expected = r#"{"time":"2019-04-28 15:24:04.309074","txn_start_ts":405888132465033227,"user":"root","host":"localhost","conn_id":0,"session_alias":"alias123","exec_retry_count":57,"exec_retry_time":0.12,"query_time":0.216905,"parse_time":0.0,"compile_time":0.0,"rewrite_time":0.0,"preproc_subqueries":0,"preproc_subqueries_time":0.0,"optimize_time":0.0,"wait_ts":0.0,"prewrite_time":0.0,"wait_prewrite_binlog_time":0.0,"commit_time":0.0,"get_commit_ts_time":0.0,"commit_backoff_time":0.0,"backoff_types":"","resolve_lock_time":0.0,"local_latch_wait_time":0.0,"write_keys":0,"write_size":0,"prewrite_region":0,"txn_retry":0,"cop_time":0.38,"process_time":0.021,"wait_time":0.0,"backoff_time":0.0,"lock_keys_time":0.0,"request_count":1,"total_keys":637,"process_keys":0,"rocksdb_delete_skipped_count":10,"rocksdb_key_skipped_count":10,"rocksdb_block_cache_hit_count":10,"rocksdb_block_read_count":10,"rocksdb_block_read_byte":100,"db":"","index_names":"","is_internal":true,"digest":"42a1c8aae6f133e934d4bf0147491709a8812ea05ff8819ec522780fe657b772","stats":"t1:1,t2:2","cop_proc_avg":0.1,"cop_proc_p90":0.2,"cop_proc_max":0.03,"cop_proc_addr":"127.0.0.1:20160","cop_wait_avg":0.05,"cop_wait_p90":0.6,"cop_wait_max":0.8,"cop_wait_addr":"0.0.0.0:20160","mem_max":70724,"disk_max":65536,"kv_total":0.0,"pd_total":0.0,"backoff_total":0.0,"write_sql_response_total":0.0,"result_rows":0,"warnings":"","backoff_detail":"Cop_backoff_regionMiss_total_times: 200 Cop_backoff_regionMiss_total_time: 0.2 Cop_backoff_regionMiss_max_time: 0.2 Cop_backoff_regionMiss_max_addr: 127.0.0.1 Cop_backoff_regionMiss_avg_time: 0.2 Cop_backoff_regionMiss_p90_time: 0.2 Cop_backoff_rpcPD_total_times: 200 Cop_backoff_rpcPD_total_time: 0.2 Cop_backoff_rpcPD_max_time: 0.2 Cop_backoff_rpcPD_max_addr: 127.0.0.1 Cop_backoff_rpcPD_avg_time: 0.2 Cop_backoff_rpcPD_p90_time: 0.2 Cop_backoff_rpcTiKV_total_times: 200 Cop_backoff_rpcTiKV_total_time: 0.2 Cop_backoff_rpcTiKV_max_time: 0.2 Cop_backoff_rpcTiKV_max_addr: 127.0.0.1 Cop_backoff_rpcTiKV_avg_time: 0.2 Cop_backoff_rpcTiKV_p90_time: 0.2","prepared":false,"succ":false,"is_explicit_txn":true,"is_write_cache_table":false,"plan_from_cache":true,"plan_from_binding":true,"has_more_results":false,"resource_group":"default","request_unit_read":2.158,"request_unit_write":2.123,"time_queued_by_rc":0.05,"tidb_cpu_time":0.01,"tikv_cpu_time":0.021,"plan":"","plan_digest":"60e9378c746d9a2be1c791047e008967cf252eb6de9167ad3aa6098fa2d523f4","binary_plan":"","prev_stmt":"update t set i = 1;","query":"select * from t;"}"#;

    let input_log = slow_log
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let result = tools::slow_log_retriever::parse_log(&[input_log]).unwrap();
    let actual = serde_json::to_string(&result[0]).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_get_batch_log_edge_cases() {
    let test_cases = vec![
        (
            // 简单单条查询
            r#"# Time: 2023-01-01T00:00:00.000000+08:00
# Query_time: 0.1
SELECT 1;"#,
            1,
        ),
        (
            // 多行查询
            r#"# Time: 2023-01-01T00:00:00.000000+08:00
# Query_time: 0.1
SELECT * FROM table
WHERE id = 1;"#,
            1,
        ),
        (
            // 多条查询
            r#"# Time: 2023-01-01T00:00:00.000000+08:00
# Query_time: 0.1
SELECT 1;
# Time: 2023-01-01T00:01:00.000000+08:00
# Query_time: 0.2
SELECT 2;"#,
            2,
        ),
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        let mut reader = io::BufReader::new(Cursor::new(input));
        let mut offset = tools::slow_log_retriever::Offset::default();
        let result = get_batch_log(&mut reader, &mut offset, 1024).unwrap();
        assert_eq!(result.len(), *expected, "测试用例 {} 失败", i);
    }
}

#[tokio::test]
async fn test_import_slow_log_to_db_concurrent() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx::mysql::MySqlPoolOptions;

    // 2. 使用 parse_data_for_slow_log 解析日志
    let retriever = tools::slow_log_retriever::SlowQueryRetriever::new(4, 64);
    let rows = retriever.parse_data_for_slow_log(
        vec!["/Users/aric/Downloads/tidb_slow_query-2025-03/tidb_slow_query.log".to_string()],
        4,
        64
    )?;
    
    // 3. 建立数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:@127.0.0.1:4000/tihc")
        .await?;

    // 4. 初始化数据库环境
    SlowQueryRow::init_db(&pool).await?;
    SlowQueryRow::drop_table(&pool).await?;
    SlowQueryRow::init_table(&pool).await?;

    // 5. 并发执行插入
    let mut tasks = Vec::new();
    for chunk in rows.chunks(1) { // 每个chunk只包含1条记录以测试并发
        let pool = pool.clone();
        let chunk = chunk.to_vec();
        tasks.push(tokio::spawn(async move {
            SlowQueryRow::batch_insert(&chunk, &pool).await
        }));
    }

    // 等待所有任务完成并检查结果
    for task in tasks {
        task.await??;
    }

    Ok(())
}