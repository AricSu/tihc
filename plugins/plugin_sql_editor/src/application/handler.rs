use crate::domain::database::{Database, DatabaseConnection};
use crate::domain::error::SqlEditorError;
use crate::infrastructure::connection_store::{ConnectionOps, ConnectionStore};
use crate::infrastructure::dml_executor;
use crate::infrastructure::table_store::TableStore;
use crate::infrastructure::table_store::TableStoreOps;
use microkernel::platform::command_registry::CommandHandler;
use std::sync::Arc;

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
    ListIndex,
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

/// 连接相关命令分发（ConnectionStore）
#[async_trait::async_trait]
impl CommandHandler for Command<ConnectionStore> {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::AddConnection => {
                tracing::debug!("handler: AddConnection entered");
                let conn = parse_args!(args, DatabaseConnection);
                self.store.add_connection(conn).await?;
                Ok(serde_json::to_value("ok")?)
            }
            Op::ListConnection => {
                tracing::debug!("handler: ListConnection entered");
                let data = self.store.list_connection().await?;
                Ok(serde_json::to_value(data)?)
            }
            Op::DeleteConnection => {
                tracing::debug!("handler: DeleteConnection entered");
                let id = parse_args!(args, u64);
                let deleted = self.store.delete_connection(id).await?;
                Ok(serde_json::to_value(deleted)?)
            }
            Op::UpdateConnection => {
                tracing::debug!("handler: UpdateConnection entered");
                let (id, update) = parse_args!(args, u64, DatabaseConnection);
                let updated = self.store.update_connection(id, update).await?;
                Ok(serde_json::to_value(updated)?)
            }
            Op::GetConnection => {
                tracing::debug!("handler: GetConnection entered");
                let id = parse_args!(args, u64);
                let conn = self
                    .store
                    .get_connection(id)
                    .await?
                    .ok_or_else(|| SqlEditorError::Other("Connection not found".to_string()))?;
                Ok(serde_json::to_value(conn)?)
            }
            Op::TestConnection => {
                tracing::debug!("handler: TestConnection entered");
                let conn = parse_args!(args, DatabaseConnection);
                let result = self.store.test_connection(&conn).await;
                match result {
                    Ok(ok) => Ok(serde_json::to_value(ok)?),
                    Err(e) => {
                        tracing::error!("test_connection error: {:?}", e);
                        Ok(serde_json::to_value(format!("连接测试异常: {}", e))?)
                    }
                }
            }
            Op::ExecuteSql => {
                tracing::debug!("handler: ExecuteSql entered");
                let (connection_id, sql) = parse_args!(args, u64, String);
                let conn = self.store.get_connection(connection_id).await?
                    .ok_or_else(|| SqlEditorError::Other("Connection not found".to_string()))?;
                let res = dml_executor::SqlExecutor::execute_sql(&conn, &sql).await?;
                Ok(serde_json::to_value(res)?)
            }
            _ => {
                tracing::debug!("handler: ConnectionStore _ (unsupported) entered");
                Ok(serde_json::to_value("unsupported")?)
            }
        }
    }
}

/// 数据库相关命令分发（DatabaseStore）
#[async_trait::async_trait]
impl CommandHandler for Command<crate::infrastructure::database_store::DatabaseStore> {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::AddDatabase => {
                let db = parse_args!(args, Database);
                self.store.backend.add(&db).await?;
                Ok(serde_json::to_value("ok")?)
            }
            Op::DeleteDatabase => {
                let db_name = parse_args!(args, &str);
                let deleted = self.store.backend.delete(db_name).await?;
                Ok(serde_json::to_value(deleted)?)
            }
            Op::UpdateDatabase => {
                let (db_name, db) = parse_args!(args, &str, Database);
                let updated = self.store.backend.update(db_name, &db).await?;
                Ok(serde_json::to_value(updated)?)
            }
            Op::ListDatabase => {
                let connection_id = parse_args!(args, u64);
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                // 动态分发 backend
                let backend: Arc<dyn crate::infrastructure::database_store::DatabaseBackend> = match &pool {
                    Some(crate::domain::database::DatabasePool::MySql(mysql_pool)) => {
                        Arc::new(crate::infrastructure::database_store::MySqlBackend { pool: Arc::clone(mysql_pool) })
                    }
                    // 可扩展更多类型
                    _ => Arc::new(crate::infrastructure::database_store::DummyBackend)
                };
                let data = backend.list(pool.unwrap()).await?;
                Ok(serde_json::to_value(data)?)
            }
            _ => Ok(serde_json::to_value("unsupported")?),
        }
    }
}
// TableStore命令处理器实现
#[async_trait::async_trait]
impl CommandHandler for Command<TableStore> {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::ListTable => {
                tracing::debug!("handler: ListTable entered");
                // args: [connection_id, schema_name]
                if args.len() < 2 {
                    return Err(SqlEditorError::Other("Missing arguments for ListTable".to_string()).into());
                }
                let connection_id = args[0].parse::<u64>().map_err(|e| SqlEditorError::Other(format!("Invalid connection_id: {}", e)))?;
                let schema_name = &args[1];
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                let tables = self.store.list(pool.unwrap(), schema_name).await?;
                Ok(serde_json::to_value(tables)?)
            }
            Op::ListColumn => {
                tracing::debug!("handler: ListColumn entered");
                // args: [connection_id, schema_name, table_name]
                if args.len() < 3 {
                    return Err(SqlEditorError::Other("Missing arguments for ListColumn".to_string()).into());
                }
                let connection_id = args[0].parse::<u64>().map_err(|e| SqlEditorError::Other(format!("Invalid connection_id: {}", e)))?;
                let schema_name = &args[1];
                let table_name = &args[2];
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                let columns = self.store.list_columns(pool.unwrap(), schema_name, table_name).await?;
                Ok(serde_json::to_value(columns)?)
            }
            Op::ListIndex => {
                tracing::debug!("handler: ListIndex entered");
                // args: [connection_id, schema_name, table_name]
                if args.len() < 3 {
                    return Err(SqlEditorError::Other("Missing arguments for ListIndex".to_string()).into());
                }
                let connection_id = args[0].parse::<u64>().map_err(|e| SqlEditorError::Other(format!("Invalid connection_id: {}", e)))?;
                let schema_name = &args[1];
                let table_name = &args[2];
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                let indexes = self.store.list_indexes(pool.unwrap(), schema_name, table_name).await?;
                Ok(serde_json::to_value(indexes)?)
            }
            Op::AddTable => {
                tracing::debug!("handler: AddTable entered");
                // args: [connection_id, table_json]
                if args.len() < 2 {
                    return Err(SqlEditorError::Other("Missing arguments for AddTable".to_string()).into());
                }
                let connection_id = args[0].parse::<u64>().map_err(|e| SqlEditorError::Other(format!("Invalid connection_id: {}", e)))?;
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                let table: crate::domain::database::TableInfo = serde_json::from_str(&args[1])?;
                self.store.add(table, pool.unwrap()).await?;
                Ok(serde_json::to_value(true)?)
            }
            Op::DeleteTable => {
                tracing::debug!("handler: DeleteTable entered");
                let connection_id = parse_args!(args, u64);
                let pool = self.store.connection_store.get_pool(connection_id);
                if pool.is_none() {
                    return Err(SqlEditorError::Other("Connection pool not found".to_string()).into());
                }
                let deleted = self.store.delete(&args[1], pool.unwrap()).await?;
                Ok(serde_json::to_value(deleted)?)
            }
            _ => {
                tracing::debug!("handler: TableStore _ (unsupported) entered");
                Ok(serde_json::to_value("unsupported")?)
            }
        }
    }
}
