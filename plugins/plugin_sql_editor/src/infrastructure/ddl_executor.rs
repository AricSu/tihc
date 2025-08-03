// use crate::domain::database::DatabaseConnection;
// use crate::domain::error::SqlEditorError;
// use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
// use serde_json::Value;

// /// DDL 执行器，专用于表/库结构变更
// pub struct DdlExecutor;

// impl DdlExecutor {
//     /// 执行 DDL，支持 MySQL/TiDB
//     pub async fn execute_ddl(conn: &DatabaseConnection, ddl: &str) -> Result<Value, SqlEditorError> {
//         let url = format!(
//             "mysql://{}:{}@{}:{}/{}",
//             conn.username, conn.password, conn.host, conn.port, conn.database
//         );
//         let pool = MySqlPoolOptions::new()
//             .max_connections(1)
//             .connect(&url)
//             .await
//             .map_err(|e| SqlEditorError::InfraCommon(crate::common::error::CommonError::from(e)))?;
//         let res = sqlx::query(ddl)
//             .execute(&pool)
//             .await
//             .map_err(|e| SqlEditorError::InfraCommon(crate::common::error::CommonError::from(e)))?;
//         Ok(serde_json::json!({ "rows_affected": res.rows_affected() }))
//     }
// }
