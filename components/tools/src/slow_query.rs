use chrono::{DateTime, FixedOffset, Local, Offset, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{MySqlPool, QueryBuilder}; // 添加这行

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlowQueryRow {
    pub time: String,
    pub txn_start_ts: u64,
    pub user: String,
    pub host: String,
    pub conn_id: u64,                       // bigint unsigned
    pub session_alias: String,              // varchar(64)
    pub exec_retry_count: u64,              // bigint unsigned
    pub exec_retry_time: f64,               // double
    pub query_time: f64,                    // double
    pub parse_time: f64,                    // double
    pub compile_time: f64,                  // double
    pub rewrite_time: f64,                  // double
    pub preproc_subqueries: u64,            // bigint unsigned
    pub preproc_subqueries_time: f64,       // double
    pub optimize_time: f64,                 // double
    pub wait_ts: f64,                       // double
    pub prewrite_time: f64,                 // double
    pub wait_prewrite_binlog_time: f64,     // double
    pub commit_time: f64,                   // double
    pub get_commit_ts_time: f64,            // double
    pub commit_backoff_time: f64,           // double
    pub backoff_types: String,              // varchar(64)
    pub resolve_lock_time: f64,             // double
    pub local_latch_wait_time: f64,         // double
    pub write_keys: i64,                    // bigint
    pub write_size: i64,                    // bigint
    pub prewrite_region: i64,               // bigint
    pub txn_retry: i64,                     // bigint
    pub cop_time: f64,                      // double
    pub process_time: f64,                  // double
    pub wait_time: f64,                     // double
    pub backoff_time: f64,                  // double
    pub lock_keys_time: f64,                // double
    pub request_count: u64,                 // bigint unsigned
    pub total_keys: u64,                    // bigint unsigned
    pub process_keys: u64,                  // bigint unsigned
    pub rocksdb_delete_skipped_count: u64,  // bigint unsigned
    pub rocksdb_key_skipped_count: u64,     // bigint unsigned
    pub rocksdb_block_cache_hit_count: u64, // bigint unsigned
    pub rocksdb_block_read_count: u64,      // bigint unsigned
    pub rocksdb_block_read_byte: u64,       // bigint unsigned
    pub db: String,                         // varchar(64)
    pub index_names: String,                // varchar(100)
    pub is_internal: bool,                  // tinyint(1)
    pub digest: String,                     // varchar(64)
    pub stats: String,                      // varchar(512)
    pub cop_proc_avg: f64,                  // double
    pub cop_proc_p90: f64,                  // double
    pub cop_proc_max: f64,                  // double
    pub cop_proc_addr: String,              // varchar(64)
    pub cop_wait_avg: f64,                  // double
    pub cop_wait_p90: f64,                  // double
    pub cop_wait_max: f64,                  // double
    pub cop_wait_addr: String,              // varchar(64)
    pub mem_max: i64,                       // bigint
    pub disk_max: i64,                      // bigint
    pub kv_total: f64,                      // double
    pub pd_total: f64,                      // double
    pub backoff_total: f64,                 // double
    pub write_sql_response_total: f64,      // double
    pub result_rows: i64,                   // bigint
    pub warnings: String,                   // longtext
    pub backoff_detail: String,             // varchar(4096)
    pub prepared: bool,                     // tinyint(1)
    pub succ: bool,                         // tinyint(1)
    pub is_explicit_txn: bool,              // tinyint(1)
    pub is_write_cache_table: bool,         // tinyint(1)
    pub plan_from_cache: bool,              // tinyint(1)
    pub plan_from_binding: bool,            // tinyint(1)
    pub has_more_results: bool,             // tinyint(1)
    pub resource_group: String,             // varchar(64)
    pub request_unit_read: f64,             // double
    pub request_unit_write: f64,            // double
    pub time_queued_by_rc: f64,             // double
    pub tidb_cpu_time: f64,                 // double
    pub tikv_cpu_time: f64,                 // double
    pub plan: String,                       // longtext
    pub plan_digest: String,                // varchar(128)
    pub binary_plan: String,                // longtext
    pub prev_stmt: String,                  // longtext
    pub query: String,                      // longtext
}

impl std::fmt::Display for SlowQueryRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

pub trait DbOps {
    async fn batch_insert(rows: &[Self], pool: &MySqlPool) -> Result<(), sqlx::Error>
    where
        Self: Sized;
    async fn init_db(pool: &MySqlPool) -> Result<(), sqlx::Error>;
    async fn init_table(pool: &MySqlPool) -> Result<(), sqlx::Error>;
    async fn drop_table(pool: &MySqlPool) -> Result<(), sqlx::Error>;
}

impl DbOps for SlowQueryRow {
    async fn batch_insert(rows: &[Self], pool: &MySqlPool) -> Result<(), sqlx::Error> {
        if rows.is_empty() {
            return Ok(());
        }

        let mut tx = pool.begin().await?;

        for chunk in rows.chunks(10) {
            let mut builder = QueryBuilder::new("INSERT INTO CLUSTER_SLOW_QUERY (");

            // 添加所有字段
            builder.push("time, txn_start_ts, user, host, conn_id, session_alias, exec_retry_count, \
                exec_retry_time, query_time, parse_time, compile_time, rewrite_time, preproc_subqueries, \
                preproc_subqueries_time, optimize_time, wait_ts, prewrite_time, wait_prewrite_binlog_time, \
                commit_time, get_commit_ts_time, commit_backoff_time, backoff_types, resolve_lock_time, \
                local_latch_wait_time, write_keys, write_size, prewrite_region, txn_retry, cop_time, \
                process_time, wait_time, backoff_time, LockKeys_time, request_count, total_keys, \
                process_keys, rocksdb_delete_skipped_count, rocksdb_key_skipped_count, \
                rocksdb_block_cache_hit_count, rocksdb_block_read_count, rocksdb_block_read_byte, \
                db, index_names, is_internal, digest, stats, cop_proc_avg, cop_proc_p90, cop_proc_max, \
                cop_proc_addr, cop_wait_avg, cop_wait_p90, cop_wait_max, cop_wait_addr, mem_max, \
                disk_max, kv_total, pd_total, backoff_total, write_sql_response_total, result_rows, \
                warnings, backoff_detail, prepared, succ, IsExplicitTxn, IsWriteCacheTable, \
                plan_from_cache, plan_from_binding, has_more_results, resource_group, request_unit_read, \
                request_unit_write, time_queued_by_rc, tidb_cpu_time, tikv_cpu_time, plan, plan_digest, \
                binary_plan, prev_stmt, query)");

            builder.push(" VALUES (");

            let mut first = true;
            for row in chunk {
                if !first {
                    builder.push("), (");
                }
                first = false;

                // 解析时间字符串为 DateTime<Utc>
                let utc_time = DateTime::parse_from_rfc3339(&row.time)
                    .expect("Failed to parse time")
                    .with_timezone(&Utc);

                // 获取本地时区偏移量
                let local_offset = Local::now().offset().fix().local_minus_utc();
                let offset = FixedOffset::east_opt(-local_offset).unwrap();

                // 根据本地时区偏移量调整时间
                let adjusted_time = utc_time
                    .with_timezone(&offset)
                    .format("%Y-%m-%d %H:%M:%S%.6f")
                    .to_string();

                builder
                    .push_bind(adjusted_time)
                    .push(",")
                    .push_bind(row.txn_start_ts)
                    .push(",")
                    .push_bind(&row.user)
                    .push(",")
                    .push_bind(&row.host)
                    .push(",")
                    .push_bind(row.conn_id)
                    .push(",")
                    .push_bind(&row.session_alias)
                    .push(",")
                    .push_bind(row.exec_retry_count)
                    .push(",")
                    .push_bind(row.exec_retry_time)
                    .push(",")
                    .push_bind(row.query_time)
                    .push(",")
                    .push_bind(row.parse_time)
                    .push(",")
                    .push_bind(row.compile_time)
                    .push(",")
                    .push_bind(row.rewrite_time)
                    .push(",")
                    .push_bind(row.preproc_subqueries)
                    .push(",")
                    .push_bind(row.preproc_subqueries_time)
                    .push(",")
                    .push_bind(row.optimize_time)
                    .push(",")
                    .push_bind(row.wait_ts)
                    .push(",")
                    .push_bind(row.prewrite_time)
                    .push(",")
                    .push_bind(row.wait_prewrite_binlog_time)
                    .push(",")
                    .push_bind(row.commit_time)
                    .push(",")
                    .push_bind(row.get_commit_ts_time)
                    .push(",")
                    .push_bind(row.commit_backoff_time)
                    .push(",")
                    .push_bind(&row.backoff_types)
                    .push(",")
                    .push_bind(row.resolve_lock_time)
                    .push(",")
                    .push_bind(row.local_latch_wait_time)
                    .push(",")
                    .push_bind(row.write_keys)
                    .push(",")
                    .push_bind(row.write_size)
                    .push(",")
                    .push_bind(row.prewrite_region)
                    .push(",")
                    .push_bind(row.txn_retry)
                    .push(",")
                    .push_bind(row.cop_time)
                    .push(",")
                    .push_bind(row.process_time)
                    .push(",")
                    .push_bind(row.wait_time)
                    .push(",")
                    .push_bind(row.backoff_time)
                    .push(",")
                    .push_bind(row.lock_keys_time)
                    .push(",")
                    .push_bind(row.request_count)
                    .push(",")
                    .push_bind(row.total_keys)
                    .push(",")
                    .push_bind(row.process_keys)
                    .push(",")
                    .push_bind(row.rocksdb_delete_skipped_count)
                    .push(",")
                    .push_bind(row.rocksdb_key_skipped_count)
                    .push(",")
                    .push_bind(row.rocksdb_block_cache_hit_count)
                    .push(",")
                    .push_bind(row.rocksdb_block_read_count)
                    .push(",")
                    .push_bind(row.rocksdb_block_read_byte)
                    .push(",")
                    .push_bind(&row.db)
                    .push(",")
                    .push_bind(&row.index_names)
                    .push(",")
                    .push_bind(row.is_internal)
                    .push(",")
                    .push_bind(&row.digest)
                    .push(",")
                    .push_bind(&row.stats)
                    .push(",")
                    .push_bind(row.cop_proc_avg)
                    .push(",")
                    .push_bind(row.cop_proc_p90)
                    .push(",")
                    .push_bind(row.cop_proc_max)
                    .push(",")
                    .push_bind(&row.cop_proc_addr)
                    .push(",")
                    .push_bind(row.cop_wait_avg)
                    .push(",")
                    .push_bind(row.cop_wait_p90)
                    .push(",")
                    .push_bind(row.cop_wait_max)
                    .push(",")
                    .push_bind(&row.cop_wait_addr)
                    .push(",")
                    .push_bind(row.mem_max)
                    .push(",")
                    .push_bind(row.disk_max)
                    .push(",")
                    .push_bind(row.kv_total)
                    .push(",")
                    .push_bind(row.pd_total)
                    .push(",")
                    .push_bind(row.backoff_total)
                    .push(",")
                    .push_bind(row.write_sql_response_total)
                    .push(",")
                    .push_bind(row.result_rows)
                    .push(",")
                    .push_bind(&row.warnings)
                    .push(",")
                    .push_bind(&row.backoff_detail)
                    .push(",")
                    .push_bind(row.prepared)
                    .push(",")
                    .push_bind(row.succ)
                    .push(",")
                    .push_bind(row.is_explicit_txn)
                    .push(",")
                    .push_bind(row.is_write_cache_table)
                    .push(",")
                    .push_bind(row.plan_from_cache)
                    .push(",")
                    .push_bind(row.plan_from_binding)
                    .push(",")
                    .push_bind(row.has_more_results)
                    .push(",")
                    .push_bind(&row.resource_group)
                    .push(",")
                    .push_bind(row.request_unit_read)
                    .push(",")
                    .push_bind(row.request_unit_write)
                    .push(",")
                    .push_bind(row.time_queued_by_rc)
                    .push(",")
                    .push_bind(row.tidb_cpu_time)
                    .push(",")
                    .push_bind(row.tikv_cpu_time)
                    .push(",")
                    .push_bind(&row.plan)
                    .push(",")
                    .push_bind(&row.plan_digest)
                    .push(",")
                    .push_bind(&row.binary_plan)
                    .push(",")
                    .push_bind(&row.prev_stmt)
                    .push(",")
                    .push_bind(&row.query);
            }
            builder.push(")");
            builder.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn init_db(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query("CREATE DATABASE IF NOT EXISTS tihc")
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn init_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS `CLUSTER_SLOW_QUERY` (
                `ID` bigint unsigned NOT NULL AUTO_RANDOM PRIMARY KEY,
                `Time` timestamp(6) NOT NULL,
                `Txn_start_ts` bigint unsigned DEFAULT NULL,
                `User` varchar(64) DEFAULT NULL,
                `Host` varchar(64) DEFAULT NULL,
                `Conn_ID` bigint unsigned DEFAULT NULL,
                `Session_alias` varchar(64) DEFAULT NULL,
                `Exec_retry_count` bigint unsigned DEFAULT NULL,
                `Exec_retry_time` double DEFAULT NULL,
                `Query_time` double DEFAULT NULL,
                `Parse_time` double DEFAULT NULL,
                `Compile_time` double DEFAULT NULL,
                `Rewrite_time` double DEFAULT NULL,
                `Preproc_subqueries` bigint unsigned DEFAULT NULL,
                `Preproc_subqueries_time` double DEFAULT NULL,
                `Optimize_time` double DEFAULT NULL,
                `Wait_TS` double DEFAULT NULL,
                `Prewrite_time` double DEFAULT NULL,
                `Wait_prewrite_binlog_time` double DEFAULT NULL,
                `Commit_time` double DEFAULT NULL,
                `Get_commit_ts_time` double DEFAULT NULL,
                `Commit_backoff_time` double DEFAULT NULL,
                `Backoff_types` longtext DEFAULT NULL,
                `Resolve_lock_time` double DEFAULT NULL,
                `Local_latch_wait_time` double DEFAULT NULL,
                `Write_keys` bigint DEFAULT NULL,
                `Write_size` bigint DEFAULT NULL,
                `Prewrite_region` bigint DEFAULT NULL,
                `Txn_retry` bigint DEFAULT NULL,
                `Cop_time` double DEFAULT NULL,
                `Process_time` double DEFAULT NULL,
                `Wait_time` double DEFAULT NULL,
                `Backoff_time` double DEFAULT NULL,
                `LockKeys_time` double DEFAULT NULL,
                `Request_count` bigint unsigned DEFAULT NULL,
                `Total_keys` bigint unsigned DEFAULT NULL,
                `Process_keys` bigint unsigned DEFAULT NULL,
                `Rocksdb_delete_skipped_count` bigint unsigned DEFAULT NULL,
                `Rocksdb_key_skipped_count` bigint unsigned DEFAULT NULL,
                `Rocksdb_block_cache_hit_count` bigint unsigned DEFAULT NULL,
                `Rocksdb_block_read_count` bigint unsigned DEFAULT NULL,
                `Rocksdb_block_read_byte` bigint unsigned DEFAULT NULL,
                `DB` longtext DEFAULT NULL,
                `Index_names` longtext DEFAULT NULL,
                `Is_internal` tinyint(1) DEFAULT NULL,
                `Digest` longtext DEFAULT NULL,
                `Stats` longtext DEFAULT NULL,
                `Cop_proc_avg` double DEFAULT NULL,
                `Cop_proc_p90` double DEFAULT NULL,
                `Cop_proc_max` double DEFAULT NULL,
                `Cop_proc_addr` longtext DEFAULT NULL,
                `Cop_wait_avg` double DEFAULT NULL,
                `Cop_wait_p90` double DEFAULT NULL,
                `Cop_wait_max` double DEFAULT NULL,
                `Cop_wait_addr` longtext DEFAULT NULL,
                `Mem_max` bigint DEFAULT NULL,
                `Disk_max` bigint DEFAULT NULL,
                `KV_total` double DEFAULT NULL,
                `PD_total` double DEFAULT NULL,
                `Backoff_total` double DEFAULT NULL,
                `Write_sql_response_total` double DEFAULT NULL,
                `Result_rows` bigint DEFAULT NULL,
                `Warnings` longtext DEFAULT NULL,
                `Backoff_Detail` longtext DEFAULT NULL,
                `Prepared` tinyint(1) DEFAULT NULL,
                `Succ` tinyint(1) DEFAULT NULL,
                `IsExplicitTxn` tinyint(1) DEFAULT NULL,
                `IsWriteCacheTable` tinyint(1) DEFAULT NULL,
                `Plan_from_cache` tinyint(1) DEFAULT NULL,
                `Plan_from_binding` tinyint(1) DEFAULT NULL,
                `Has_more_results` tinyint(1) DEFAULT NULL,
                `Resource_group` longtext DEFAULT NULL,
                `Request_unit_read` double DEFAULT NULL,
                `Request_unit_write` double DEFAULT NULL,
                `Time_queued_by_rc` double DEFAULT NULL,
                `Tidb_cpu_time` double DEFAULT NULL,
                `Tikv_cpu_time` double DEFAULT NULL,
                `Plan` longtext DEFAULT NULL,
                `Plan_digest` longtext DEFAULT NULL,
                `Binary_plan` longtext DEFAULT NULL,
                `Prev_stmt` longtext DEFAULT NULL,
                `Query` longtext DEFAULT NULL
                )"#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn drop_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS CLUSTER_SLOW_QUERY")
            .execute(pool)
            .await?;
        Ok(())
    }
}
