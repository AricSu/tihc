use crate::infrastructure::database_store::DatabaseStore;
impl CommandHandler for Command<DatabaseStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();
        match self.op {
            Op::AddDatabase => {
                let mut db: crate::domain::database::Database = serde_json::from_str(&args[0])?;
                db.created_at = Some(chrono::Utc::now().to_rfc3339());
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[1])?;
                match conn.engine.as_str() {
                    "mysql" => {
                        rt.block_on(DatabaseStore::with_mysql(&conn).add(&db))?;
                        Ok(serde_json::json!({"status": "success"}))
                    }
                    "postgres" => {
                        rt.block_on(DatabaseStore::with_postgres(&conn).add(&db))?;
                        Ok(serde_json::json!({"status": "success"}))
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
                    "postgres" => {
                        let list = rt.block_on(DatabaseStore::with_postgres(&conn).list())?;
                        Ok(serde_json::to_value(list)?)
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            Op::DeleteDatabase => {
                let name = &args[0];
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[1])?;
                match conn.engine.as_str() {
                    "mysql" => {
                        let success = rt.block_on(DatabaseStore::with_mysql(&conn).delete(name))?;
                        Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
                    }
                    "postgres" => {
                        let success =
                            rt.block_on(DatabaseStore::with_postgres(&conn).delete(name))?;
                        Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            Op::GetDatabase => {
                let name = &args[0];
                let conn: crate::domain::database::DatabaseConnection =
                    serde_json::from_str(&args[1])?;
                match conn.engine.as_str() {
                    "mysql" => match rt.block_on(DatabaseStore::with_mysql(&conn).get(name))? {
                        Some(db) => Ok(serde_json::to_value(db)?),
                        None => Ok(serde_json::json!({"status": "not_found"})),
                    },
                    "postgres" => {
                        match rt.block_on(DatabaseStore::with_postgres(&conn).get(name))? {
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
                    "postgres" => {
                        let success =
                            rt.block_on(DatabaseStore::with_postgres(&conn).update(name, &update))?;
                        Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
                    }
                    _ => Ok(serde_json::json!({"status": "unsupported_engine"})),
                }
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}
use crate::domain::database::{Column, DatabaseConnection, Table};
use crate::domain::{ConnectionListResult, StatusResult, TableListResult};
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use core::platform::command_registry::CommandHandler;
// ...existing code...
// 通用命令操作枚举
pub enum Op {
    AddConnection,
    ListConnection,
    DeleteConnection,
    TestConnection,
    AddTable,
    ListTable,
    GetTable,
    UpdateTable,
    DeleteTable,
    AddColumn,
    DeleteColumn,
    // Database/schema ops
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
                use sqlx::{Connection, MySqlConnection, PgConnection, SqliteConnection};
                let conn: DatabaseConnection = serde_json::from_str(&args[0])?;
                let engine = conn.engine.as_str();
                let host = conn.host.as_str();
                let port = conn.port;
                let username = conn.username.as_str();
                let password = conn.password.as_deref().unwrap_or("");
                let database = conn.database.as_deref().unwrap_or("");
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(async {
                        match engine {
                            "postgres" => {
                                let url = format!(
                                    "postgres://{}:{}@{}:{}/{}",
                                    username, password, host, port, database
                                );
                                match PgConnection::connect(&url).await {
                                    Ok(_) => StatusResult {
                                        status: "success".to_string(),
                                        message: None,
                                    },
                                    Err(e) => StatusResult {
                                        status: "failed".to_string(),
                                        message: Some(format!("{}", e)),
                                    },
                                }
                            }
                            "mysql" => {
                                let url = format!(
                                    "mysql://{}:{}@{}:{}/{}",
                                    username, password, host, port, database
                                );
                                match MySqlConnection::connect(&url).await {
                                    Ok(_) => StatusResult {
                                        status: "success".to_string(),
                                        message: None,
                                    },
                                    Err(e) => StatusResult {
                                        status: "failed".to_string(),
                                        message: Some(format!("{}", e)),
                                    },
                                }
                            }
                            "sqlite" => {
                                let url = database;
                                match SqliteConnection::connect(url).await {
                                    Ok(_) => StatusResult {
                                        status: "success".to_string(),
                                        message: None,
                                    },
                                    Err(e) => StatusResult {
                                        status: "failed".to_string(),
                                        message: Some(format!("{}", e)),
                                    },
                                }
                            }
                            _ => StatusResult {
                                status: "failed".to_string(),
                                message: Some("Unsupported engine".to_string()),
                            },
                        }
                    })
                });
                Ok(serde_json::to_value(result)?)
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}
impl CommandHandler for Command<TableStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::AddTable => {
                let table: Table = serde_json::from_str(&args[0])?;
                self.store.add(table);
                Ok(serde_json::json!({"status": "success"}))
            }
            Op::ListTable => {
                let list = self.store.list();
                Ok(serde_json::to_value(TableListResult { data: list })?)
            }
            Op::GetTable => {
                let table_name = &args[0];
                match self.store.get(table_name) {
                    Some(table) => Ok(serde_json::to_value(table)?),
                    None => Ok(serde_json::json!({"status": "not_found"})),
                }
            }
            Op::UpdateTable => {
                let table_name = &args[0];
                let update: Table = serde_json::from_str(&args[1])?;
                let success = self.store.update(table_name, update);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            Op::DeleteTable => {
                let table_name = &args[0];
                let success = self.store.delete(table_name);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            Op::AddColumn => {
                let table_name = &args[0];
                let column: Column = serde_json::from_str(&args[1])?;
                let success = self.store.add_column(table_name, column);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            Op::DeleteColumn => {
                let table_name = &args[0];
                let column_name = &args[1];
                let success = self.store.delete_column(table_name, column_name);
                Ok(serde_json::json!({"status": if success {"success"} else {"not_found"}}))
            }
            _ => Ok(serde_json::json!({"status": "not_supported"})),
        }
    }
}

// 测试连接命令
pub struct TestConnectionCommand;
impl CommandHandler for TestConnectionCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        use sqlx::{Connection, MySqlConnection, PgConnection, SqliteConnection};
        use tokio::runtime::Runtime;
        let conn: DatabaseConnection = serde_json::from_str(&args[0])?;
        let engine = conn.engine.as_str();
        let host = conn.host.as_str();
        let port = conn.port;
        let username = conn.username.as_str();
        let password = conn.password.as_deref().unwrap_or("");
        let database = conn.database.as_deref().unwrap_or("");
        let result = Runtime::new().unwrap().block_on(async {
            match engine {
                "postgres" => {
                    let url = format!(
                        "postgres://{}:{}@{}:{}/{}",
                        username, password, host, port, database
                    );
                    match PgConnection::connect(&url).await {
                        Ok(_) => StatusResult {
                            status: "success".to_string(),
                            message: None,
                        },
                        Err(e) => StatusResult {
                            status: "failed".to_string(),
                            message: Some(format!("{}", e)),
                        },
                    }
                }
                "mysql" => {
                    let url = format!(
                        "mysql://{}:{}@{}:{}/{}",
                        username, password, host, port, database
                    );
                    match MySqlConnection::connect(&url).await {
                        Ok(_) => StatusResult {
                            status: "success".to_string(),
                            message: None,
                        },
                        Err(e) => StatusResult {
                            status: "failed".to_string(),
                            message: Some(format!("{}", e)),
                        },
                    }
                }
                "sqlite" => {
                    let url = database;
                    match SqliteConnection::connect(url).await {
                        Ok(_) => StatusResult {
                            status: "success".to_string(),
                            message: None,
                        },
                        Err(e) => StatusResult {
                            status: "failed".to_string(),
                            message: Some(format!("{}", e)),
                        },
                    }
                }
                _ => StatusResult {
                    status: "failed".to_string(),
                    message: Some("Unsupported engine".to_string()),
                },
            }
        });
        Ok(serde_json::to_value(result)?)
    }
}

// SQL 执行命令
pub struct ExecuteSqlCommand {}
impl CommandHandler for ExecuteSqlCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let sql = &args[0];
        if sql.trim().to_lowercase().starts_with("select") {
            Ok(
                serde_json::json!({"status": "success", "data": [{"id": 1, "name": "Alice", "email": "alice@example.com"}]}),
            )
        } else {
            Ok(serde_json::json!({"status": "success", "message": "SQL executed"}))
        }
    }
}
