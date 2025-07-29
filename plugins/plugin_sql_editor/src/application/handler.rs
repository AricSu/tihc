// 获取连接命令
pub struct GetConnectionCommand {
    pub store: Arc<ConnectionStore>,
}
impl CommandHandler for GetConnectionCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        tracing::info!(target: "plugin_sql_editor", "[GetConnectionCommand] args={:?}", args);
        let id: u64 = args[0].parse()?;
        let list = self.store.list();
        if let Some(conn) = list.iter().find(|c| c.id == id) {
            tracing::info!(target: "plugin_sql_editor", "[GetConnectionCommand] Found connection: id={}, name={}", conn.id, conn.name);
            Ok(serde_json::to_value(conn)?)
        } else {
            tracing::warn!(target: "plugin_sql_editor", "[GetConnectionCommand] Connection not found: id={}", id);
            let status = StatusResult {
                status: "not_found".to_string(),
                message: Some("Connection not found".to_string()),
            };
            Ok(serde_json::to_value(status)?)
        }
    }
}

// 更新连接命令
pub struct UpdateConnectionCommand {
    pub store: Arc<ConnectionStore>,
}
impl CommandHandler for UpdateConnectionCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        tracing::info!(target: "plugin_sql_editor", "[UpdateConnectionCommand] args={:?}", args);
        let id: u64 = args[0].parse()?;
        let update: DatabaseConnection = serde_json::from_str(&args[1])?;
        let mut conns = self.store.connections.lock().unwrap();
        if let Some(conn) = conns.iter_mut().find(|c| c.id == id) {
            tracing::info!(target: "plugin_sql_editor", "[UpdateConnectionCommand] Update connection: id={}, name={}", id, update.name);
            *conn = update;
            let status = StatusResult {
                status: "success".to_string(),
                message: None,
            };
            Ok(serde_json::to_value(status)?)
        } else {
            tracing::warn!(target: "plugin_sql_editor", "[UpdateConnectionCommand] Connection not found: id={}", id);
            let status = StatusResult {
                status: "not_found".to_string(),
                message: Some("Connection not found".to_string()),
            };
            Ok(serde_json::to_value(status)?)
        }
    }
}
// 新增表命令
pub struct AddTableCommand {
    pub store: Arc<TableStore>,
}
impl CommandHandler for AddTableCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        tracing::info!(target: "plugin_sql_editor", "[AddTableCommand] args={:?}", args);
        let table: Table = serde_json::from_str(&args[0])?;
        tracing::info!(target: "plugin_sql_editor", "[AddTableCommand] Add table: name={}", table.name);
        self.store.add(table);
        let status = StatusResult {
            status: "success".to_string(),
            message: None,
        };
        Ok(serde_json::to_value(status)?)
    }
}

// 删除表命令
pub struct DeleteTableCommand {
    pub store: Arc<TableStore>,
}
impl CommandHandler for DeleteTableCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        tracing::info!(target: "plugin_sql_editor", "[DeleteTableCommand] args={:?}", args);
        let table_name = &args[0];
        let mut tables = self.store.tables.lock().unwrap();
        let len_before = tables.len();
        tables.retain(|t| t.name != *table_name);
        let success = len_before != tables.len();
        if success {
            tracing::info!(target: "plugin_sql_editor", "[DeleteTableCommand] Deleted table: name={}", table_name);
        } else {
            tracing::warn!(target: "plugin_sql_editor", "[DeleteTableCommand] Table not found: name={}", table_name);
        }
        let status = StatusResult {
            status: if success {"success".to_string()} else {"not_found".to_string()},
            message: None,
        };
        Ok(serde_json::to_value(status)?)
    }
}
use std::sync::Arc;
use core::platform::command_registry::CommandHandler;
use serde_json;
use crate::domain::{ConnectionListResult, TableListResult, StatusResult, SqlQueryResult};
use crate::domain::database::{DatabaseConnection, Table, Column};
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use tracing;

// 通用命令处理器 trait，支持泛型存储和操作
pub trait StoreCommand<T>: Send + Sync {
    fn store(&self) -> &Arc<T>;
}

// 泛型命令结构体
pub struct Command<T: Send + Sync> {
    pub store: Arc<T>,
}

impl<T: Send + Sync> StoreCommand<T> for Command<T> {
    fn store(&self) -> &Arc<T> { &self.store }
}

pub enum ConnectionOp {
    Create,
    List,
}

pub struct ConnectionCommand {
    pub store: Arc<ConnectionStore>,
    pub op: ConnectionOp,
}

impl CommandHandler for ConnectionCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            ConnectionOp::Create => {
                tracing::info!(target: "plugin_sql_editor", "[ConnectionCommand] Create called, args={:?}", args);
                let mut req: DatabaseConnection = serde_json::from_str(&args[0])?;
                req.id = chrono::Utc::now().timestamp_millis() as u64;
                req.created_at = chrono::Utc::now().to_rfc3339();
                tracing::info!(target: "plugin_sql_editor", "[ConnectionCommand] Created connection: id={}, name={}", req.id, req.name);
                self.store.add(req);
                let status = StatusResult { status: "success".to_string(), message: None };
                Ok(serde_json::to_value(status)?)
            }
            ConnectionOp::List => {
                tracing::info!(target: "plugin_sql_editor", "[ConnectionCommand] List called");
                let list = self.store.list();
                tracing::info!(target: "plugin_sql_editor", "[ConnectionCommand] List result count={}", list.len());
                let result = ConnectionListResult { data: list };
                Ok(serde_json::to_value(result)?)
            }
        }
    }
}
// 列表命令（表）
impl CommandHandler for Command<TableStore> {
    fn handle(&self, _args: &[String]) -> anyhow::Result<serde_json::Value> {
        let list = self.store.list();
        let result = TableListResult { data: list };
        Ok(serde_json::to_value(result)?)
    }
}

// 删除命令（连接/表字段）
pub struct DeleteCommand<T> {
    pub store: Arc<T>,
}
impl CommandHandler for DeleteCommand<ConnectionStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let id: u64 = args[0].parse()?;
        let success = self.store.delete(id);
        let status = StatusResult {
            status: if success {"success".to_string()} else {"not_found".to_string()},
            message: None,
        };
        Ok(serde_json::to_value(status)?)
    }
}
impl CommandHandler for DeleteCommand<TableStore> {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let table_name = &args[0];
        let column_name = &args[1];
        let success = self.store.delete_column(table_name, column_name);
        let status = StatusResult {
            status: if success {"success".to_string()} else {"not_found".to_string()},
            message: None,
        };
        Ok(serde_json::to_value(status)?)
    }
}

// 增加字段命令
pub struct AddColumnCommand {
    pub store: Arc<TableStore>,
}
impl CommandHandler for AddColumnCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let table_name = &args[0];
        let column: Column = serde_json::from_str(&args[1])?;
        let success = self.store.add_column(table_name, column);
        let status = StatusResult {
            status: if success {"success".to_string()} else {"not_found".to_string()},
            message: None,
        };
        Ok(serde_json::to_value(status)?)
    }
}

// SQL 执行命令
pub struct ExecuteSqlCommand;
impl CommandHandler for ExecuteSqlCommand {
    fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        let sql = &args[0];
        // mock: select 返回一行，其他返回成功
        if sql.trim().to_lowercase().starts_with("select") {
            let result = SqlQueryResult {
                status: "success".to_string(),
                data: Some(vec![serde_json::json!({"id": 1, "name": "Alice", "email": "alice@example.com"})]),
                message: None,
            };
            Ok(serde_json::to_value(result)?)
        } else {
            let result = SqlQueryResult {
                status: "success".to_string(),
                data: None,
                message: Some("SQL executed".to_string()),
            };
            Ok(serde_json::to_value(result)?)
        }
    }
}
