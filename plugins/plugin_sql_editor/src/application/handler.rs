use crate::application::service::SqlEditorService;
use crate::domain::ConnectionListResult;
use crate::domain::database::Database;
use crate::domain::database::DatabaseConnection;
use crate::infrastructure::database_store::DatabaseStore;
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use crate::domain::error::SqlEditorError;
use microkernel::platform::command_registry::CommandHandler;
use sqlx::Connection;
use tokio::runtime::Handle;

pub enum Op {
    AddConnection,
    ListConnection,
    DeleteConnection,
    TestConnection,
    UpdateConnection,
    GetConnection,
    AddTable,
    ListTable,
    DeleteTable,
    AddColumn,
    ListColumn,
    DeleteColumn,
    AddDatabase,
    ListDatabase,
    DeleteDatabase,
    GetDatabase,
    UpdateDatabase,
    ExecuteSql,
}

pub struct Command<T> {
    pub store: std::sync::Arc<T>,
    pub op: Op,
}

impl CommandHandler for Command<DatabaseStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::ExecuteSql => {
                let (connection_id, sql) = parse_args!(args, u64, String);
                let res = SqlEditorService::execute_sql(connection_id, &sql)?;
                Ok(serde_json::to_value(json_resp!(success, res))?)
            }
            Op::AddDatabase => {
                let (db, conn) = parse_args!(args, Database, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value(json_resp!(UnsupportedEngine))?);
                }
                let url = if conn.use_tls {
                    format!(
                        "mysql://{}:{}@{}:{}/{}?ssl-mode=REQUIRED",
                        conn.username,
                        conn.password.as_deref().unwrap_or(""),
                        conn.host,
                        conn.port,
                        conn.database.as_deref().unwrap_or("")
                    )
                } else {
                    format!(
                        "mysql://{}:{}@{}:{}/{}",
                        conn.username,
                        conn.password.as_deref().unwrap_or(""),
                        conn.host,
                        conn.port,
                        conn.database.as_deref().unwrap_or("")
                    )
                };
                let ok = Handle::current()
                    .block_on(async { sqlx::MySqlConnection::connect(&url).await.is_ok() });
                if !ok {
                    return Ok(serde_json::to_value(json_resp!(Failed, "数据库连接失败"))?);
                }
                Handle::current()
                    .block_on(DatabaseStore::with_mysql(&conn).add(&db))?;
                Ok(serde_json::to_value(json_resp!(Success))?)
            }
            Op::ListDatabase => {
                let conn = parse_args!(args, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value(json_resp!(UnsupportedEngine))?);
                }
                let list = Handle::current()
                    .block_on(DatabaseStore::with_mysql(&conn).list())?;
                Ok(serde_json::to_value(json_resp!(success, list))?)
            }
            Op::GetDatabase => {
                let (name, conn) = parse_args!(args, &str, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value(json_resp!(UnsupportedEngine))?);
                }
                let db = Handle::current()
                    .block_on(DatabaseStore::with_mysql(&conn).get(name))?;
                Ok(serde_json::to_value(match db {
                    Some(db) => json_resp!(success, db),
                    None => json_resp!(NotFound, db),
                })?)
            }
            Op::UpdateDatabase => {
                let (name, update, conn) = parse_args!(args, &str, Database, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value(json_resp!(UnsupportedEngine))?);
                }
                let ok = Handle::current()
                    .block_on(DatabaseStore::with_mysql(&conn).update(name, &update))?;
                Ok(serde_json::to_value(if ok {
                    json_resp!(Success)
                } else {
                    json_resp!(NotFound)
                })?)
            }
            _ => Ok(serde_json::to_value(json_resp!(
                Failed,
                "Operation not supported"
            ))?),
        }
    }
}

impl CommandHandler for Command<ConnectionStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::AddConnection => {
                let mut req = parse_args!(args, DatabaseConnection);
                req.id = chrono::Utc::now().timestamp_millis() as u64;
                req.created_at = chrono::Utc::now().to_rfc3339();
                self.store.add(req)?;
                return Ok(serde_json::to_value(json_resp!(Success))?);
            }
            Op::ListConnection => {
                let data = self.store.list()?;
                return Ok(serde_json::to_value(json_resp!(success, ConnectionListResult { data }))?);
            }
            Op::DeleteConnection => {
                let id = parse_args!(args, u64);
                let deleted = self.store.delete(id)?;
                return Ok(serde_json::to_value(if deleted {
                    json_resp!(Success)
                } else {
                    json_resp!(NotFound)
                })?);
            }
            Op::UpdateConnection => {
                let (id, update) = parse_args!(args, u64, DatabaseConnection);
                let updated = self.store.update(id, update)?;
                return Ok(serde_json::to_value(if updated {
                    json_resp!(Success)
                } else {
                    json_resp!(NotFound)
                })?);
            }
            Op::GetConnection => {
                let id = parse_args!(args, u64);
                let conn = self.store.get(id)?.ok_or_else(|| SqlEditorError::Other("Connection not found".to_string()))?;
                return Ok(serde_json::to_value(json_resp!(success, conn))?);
            }
            _ => {
                return Ok(serde_json::to_value(json_resp!(
                    Failed,
                    "Operation not supported"
                ))?);
            }
        }
    }
}
// TableStore命令处理器实现
impl CommandHandler for Command<TableStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::ListTable => Ok(serde_json::to_value(json_resp!(
                success,
                self.store.list()
            ))?),
            Op::AddTable => {
                self.store.add(serde_json::from_str(&args[0])?);
                Ok(serde_json::to_value(json_resp!(Success))?)
            }
            Op::DeleteTable => Ok(serde_json::to_value(if self.store.delete(&args[0]) {
                json_resp!(Success)
            } else {
                json_resp!(NotFound)
            })?),
            _ => Ok(serde_json::to_value(json_resp!(
                Failed,
                "Operation not supported"
            ))?),
        }
    }
}
