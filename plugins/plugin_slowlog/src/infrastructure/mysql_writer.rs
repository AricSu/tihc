use crate::domain::connection::Connection;
pub use crate::domain::table::SlowQueryRow;
use anyhow::Result;
use sqlx::MySqlPool;
use sqlx::QueryBuilder;

/// 根据 Connection 创建 MySQL 连接池
pub async fn get_mysql_pool(conn: &Connection) -> Result<MySqlPool> {
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        conn.username,
        conn.password.as_deref().unwrap_or(""),
        conn.host,
        conn.port,
        conn.database.as_deref().unwrap_or("tihc")
    );
    Ok(MySqlPool::connect(&db_url).await?)
}

/// 初始化数据库和 SLOW_QUERY 表（如不存在）
pub async fn init_db_and_table(pool: &MySqlPool) -> anyhow::Result<()> {
    // 创建数据库
    sqlx::query("CREATE DATABASE IF NOT EXISTS tihc;")
        .execute(pool)
        .await?;
    // 切换数据库
    sqlx::query("USE tihc;").execute(pool).await?;
    // 创建表
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS tihc.SLOW_QUERY (
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

/// 批量写入 slowlog rows 到 MySQL（安全参数绑定，无需 tz）
pub async fn write_slowlog_rows(rows: &[SlowQueryRow], pool: &MySqlPool) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut tx = pool.begin().await?;
    for chunk in rows.chunks(100) {
        let mut builder = QueryBuilder::new(format!("INSERT INTO {}.SLOW_QUERY (", "tihc"));

        // Add all fields
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

            // let adjusted_time = parse_time(&row.time, tz);

            builder
                .push_bind(&row.time)
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
