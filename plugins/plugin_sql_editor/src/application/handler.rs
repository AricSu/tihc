use crate::domain::database::{DatabaseConnection, Table};
use crate::domain::{ConnectionListResult, StatusResult};
use crate::infrastructure::database_store::DatabaseStore;
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use core::platform::command_registry::CommandHandler;

// ...existing code...
// 通用命令操作枚举
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
}

// 泛型命令处理器
pub struct Command<T> {
    pub store: std::sync::Arc<T>,
    pub op: Op,
}

impl CommandHandler for Command<DatabaseStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();
        match self.op {
            Op::AddDatabase => {
                use sqlx::Connection;
                use sqlx::MySqlConnection;
                use tracing::info;
                let db: crate::domain::database::Database = serde_json::from_str(&args[0])?;
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[1])?;
                let engine = conn.engine.as_str();
                let host = conn.host.as_str();
                let port = conn.port;
                let username = conn.username.as_str();
                let password = conn.password.as_deref().unwrap_or("");
                let database = conn.database.as_deref().unwrap_or("");
                let use_tls = conn.use_tls;
                match engine {
                    "mysql" => {
                        let mut url = format!(
                            "mysql://{}:{}@{}:{}/{}",
                            username, password, host, port, database
                        );
                        if use_tls {
                            url = format!("{}?ssl-mode=REQUIRED", url);
                        }
                        info!(target: "sql_editor", "[AddDatabase] mysql url={}", url);
                        let test_result = tokio::runtime::Handle::current().block_on(async {
                            match MySqlConnection::connect(&url).await {
                                Ok(_) => {
                                    info!(target: "sql_editor", "[AddDatabase] mysql connect success");
                                    true
                                }
                                Err(e) => {
                                    info!(target: "sql_editor", "[AddDatabase] mysql connect failed: {}", e);
                                    false
                                }
                            }
                        });
                        if test_result {
                            rt.block_on(DatabaseStore::with_mysql(&conn).add(&db))?;
                            Ok(serde_json::json!({"status": "success"}))
                        } else {
                            Ok(serde_json::json!({"status": "failed", "message": "数据库连接失败"}))
                        }
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            Op::ListDatabase => {
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[0])?;
                match conn.engine.as_str() {
                    "mysql" => {
                        let list = rt.block_on(DatabaseStore::with_mysql(&conn).list())?;
                        Ok(serde_json::to_value(list)?)
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            Op::GetDatabase => {
                let name = &args[0];
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[1])?;
                match conn.engine.as_str() {
                    "mysql" => {
                        let result = rt.block_on(DatabaseStore::with_mysql(&conn).get(name))?;
                        match result {
                            Some(db) => Ok(serde_json::to_value(db)?),
                            None => Ok(serde_json::json!({"status": "not_found"})),
                        }
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            Op::UpdateDatabase => {
                let name = &args[0];
                let update: crate::domain::database::Database = serde_json::from_str(&args[1])?;
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[2])?;
                match conn.engine.as_str() {
                    "mysql" => {
                        let success =
                            rt.block_on(DatabaseStore::with_mysql(&conn).update(name, &update))?;
                        Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}

impl CommandHandler for Command<ConnectionStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::AddConnection => {
                let mut req: DatabaseConnection = serde_json::from_str(&args[0])?;
                req.id = chrono::Utc::now().timestamp_millis() as u64;
                req.created_at = chrono::Utc::now().to_rfc3339();
                self.store.add(req);
                Ok(serde_json::json!({"status": "success"}))
            }
            Op::ListConnection => {
                let list = self.store.list();
                Ok(serde_json::to_value(ConnectionListResult { data: list })?)
            }
            Op::DeleteConnection => {
                let id: u64 = args[0].parse()?;
                let success = self.store.delete(id);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            Op::TestConnection => {
                use sqlx::Connection;
                use sqlx::mysql::MySqlConnection;
                use tracing::info;
                let conn: DatabaseConnection = serde_json::from_str(&args[0])?;
                let engine = conn.engine.as_str();
                let host = conn.host.as_str();
                let port = conn.port;
                let username = conn.username.as_str();
                let password = conn.password.as_deref().unwrap_or("");
                let database = conn.database.as_deref().unwrap_or("");
                let use_tls = conn.use_tls;
                let ca_cert_path = conn.ca_cert_path.as_deref();
                info!(target: "sql_editor", "[TestConnection] engine={}, host={}, port={}, user={}, db={}, use_tls={}, ca_cert_path={:?}", engine, host, port, username, database, use_tls, ca_cert_path);
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        match engine {
                            "mysql" | "tidb" => {
                                let mut url = format!(
                                    "mysql://{}:{}@{}:{}/{}",
                                    username, password, host, port, database
                                );
                                if use_tls {
                                    url = format!("{}?ssl-mode=REQUIRED", url);
                                }
                                info!(target: "sql_editor", "[TestConnection] mysql/tidb url={}", url);
                                match MySqlConnection::connect(&url).await {
                                    Ok(_) => {
                                        info!(target: "sql_editor", "[TestConnection] mysql/tidb success");
                                        StatusResult {
                                            status: "success".to_string(),
                                            message: None,
                                        }
                                    }
                                    Err(e) => {
                                        info!(target: "sql_editor", "[TestConnection] mysql/tidb failed: {}", e);
                                        StatusResult {
                                            status: "failed".to_string(),
                                            message: Some(format!("{}", e)),
                                        }
                                    }
                                }
                            }
                            _ => {
                                info!(target: "sql_editor", "[TestConnection] unsupported engine: {}", engine);
                                StatusResult {
                                    status: "failed".to_string(),
                                    message: Some("Unsupported engine".to_string()),
                                }
                            }
                        }
                    })
                });
                info!(target: "sql_editor", "[TestConnection] result={:?}", result);
                Ok(serde_json::to_value(result)?)
            }
            Op::UpdateConnection => {
                // args[0]: conn_id, args[1]: json
                let id: u64 = args[0].parse()?;
                let req: DatabaseConnection = serde_json::from_str(&args[1])?;
                let success = self.store.update(id, req);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            Op::GetConnection => {
                // 只允许一个 connectionId 参数
                if args.len() != 1 {
                    return Err(anyhow::anyhow!("Exactly one connectionId required"));
                }
                let id = args[0].parse::<u64>().map_err(|_| anyhow::anyhow!("Invalid connectionId"))?;
                let conns = self.store.connections.lock().unwrap();
                if let Some(conn) = conns.iter().find(|c| c.id == id) {
                    Ok(serde_json::to_value(conn)?)
                } else {
                    Err(anyhow::anyhow!("Connection not found"))
                }
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}
// TableStore命令处理器实现
impl CommandHandler for Command<TableStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::ListTable => {
                let list = self.store.list();
                Ok(serde_json::json!({"status": "success", "data": list}))
            }
            Op::AddTable => {
                let table: Table = serde_json::from_str(&args[0])?;
                self.store.add(table);
                Ok(serde_json::json!({"status": "success"}))
            }
            Op::DeleteTable => {
                let table_name = &args[0];
                let success = self.store.delete(table_name);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}

// // SQL 执行命令
// pub struct ExecuteSqlCommand {}
// impl CommandHandler for ExecuteSqlCommand {
//     fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
//         let sql = &args[0];
//         if sql.trim().to_lowercase().starts_with("select") {
//             Ok(
//                 serde_json::json!({"status": "success", "data": [{"id": 1, "name": "Alice", "email": "alice@example.com"}]}),
//             )
//         } else {
//             Err(anyhow::anyhow!("Unsupported SQL statement"))
//         }
//     }
// }
