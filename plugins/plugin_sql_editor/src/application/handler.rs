use crate::application::service::SqlEditorService;
use crate::domain::database::Database;
use crate::domain::database::DatabaseConnection;
use crate::domain::error::SqlEditorError;
use crate::infrastructure::database_store::DatabaseStore;
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use microkernel::platform::command_registry::CommandHandler;

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

#[async_trait::async_trait]
impl CommandHandler for Command<DatabaseStore> {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::ExecuteSql => {
                tracing::debug!("handler: ExecuteSql entered");
                let (connection_id, sql) = parse_args!(args, u64, String);
                let res = SqlEditorService::execute_sql(connection_id, &sql)?;
                Ok(serde_json::to_value(res)?)
            }
            Op::TestConnection => {
                tracing::debug!("handler: TestConnection entered");
                let conn = parse_args!(args, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value("unsupported_engine")?);
                }
                let result = self.store.test_connection(&conn).await;
                match result {
                    Ok(ok) => Ok(serde_json::to_value(ok)?),
                    Err(e) => {
                        tracing::error!("test_connection error: {:?}", e);
                        Ok(serde_json::to_value(format!("连接测试异常: {}", e))?)
                    }
                }
            }
            Op::AddConnection => {
                tracing::debug!("handler: AddConnection entered");
                let db = parse_args!(args, Database);
                self.store.add(&db).await?;
                return Ok(serde_json::to_value("ok")?);
            }
            Op::ListConnection => {
                tracing::debug!("handler: ListConnection entered");
                let data = self.store.list().await?;
                return Ok(serde_json::to_value(data)?);
            }
            Op::DeleteConnection => {
                tracing::debug!("handler: DeleteConnection entered");
                let db_name = parse_args!(args, &str);
                let deleted = self.store.delete(db_name).await?;
                tracing::debug!("handler: ListDatabase branch entered");
                return Ok(serde_json::to_value(deleted)?);
            }
            Op::UpdateConnection => {
                tracing::debug!("handler: UpdateConnection entered");
                let (db_name, db) = parse_args!(args, &str, Database);
                let updated = self.store.update(db_name, &db).await?;
                tracing::debug!("handler: GetDatabase branch entered");
                return Ok(serde_json::to_value(updated)?);
            }
            Op::GetConnection => {
                tracing::debug!("handler: GetConnection entered");
                let db_name = parse_args!(args, &str);
                let conn = self
                    .store
                    .get(db_name).await?
                    .ok_or_else(|| SqlEditorError::Other("Connection not found".to_string()))?;
                tracing::debug!("handler: UpdateDatabase branch entered");
                return Ok(serde_json::to_value(conn)?);
            }
            _ => {
                tracing::debug!("handler: ConnectionStore _ (unsupported) entered");
                return Ok(serde_json::to_value("unsupported")?);
            }
        }
    }
}

#[async_trait::async_trait]
impl CommandHandler for Command<ConnectionStore> {
    async fn handle(&self, args: &[String]) -> anyhow::Result<serde_json::Value> {
        match self.op {
            Op::TestConnection => {
                tracing::debug!("handler: TestConnection entered");
                let conn = parse_args!(args, DatabaseConnection);
                if conn.engine != "mysql" && conn.engine != "tidb" {
                    return Ok(serde_json::to_value("unsupported_engine")?);
                }
                let result = DatabaseStore::with_mysql(&conn).test_connection(&conn).await;
                match result {
                    Ok(ok) => Ok(serde_json::to_value(ok)?),
                    Err(e) => {
                        tracing::error!("test_connection error: {:?}", e);
                        Ok(serde_json::to_value(format!("连接测试异常: {}", e))?)
                    }
                }
            }
            Op::AddConnection => {
                tracing::debug!("handler: AddConnection entered");
                let mut req = parse_args!(args, DatabaseConnection);
                req.id = chrono::Utc::now().timestamp_millis() as u64;
                req.created_at = chrono::Utc::now().to_rfc3339();
                self.store.add(req)?;
                return Ok(serde_json::to_value("ok")?);
            }
            Op::ListConnection => {
                tracing::debug!("handler: ListConnection entered");
                let data = self.store.list()?;
                return Ok(serde_json::to_value(data)?);
            }
            Op::DeleteConnection => {
                tracing::debug!("handler: DeleteConnection entered");
                let id = parse_args!(args, u64);
                let deleted = self.store.delete(id)?;
                tracing::debug!("handler: ListDatabase branch entered");
                return Ok(serde_json::to_value(deleted)?);
            }
            Op::UpdateConnection => {
                tracing::debug!("handler: UpdateConnection entered");
                let (id, update) = parse_args!(args, u64, DatabaseConnection);
                let updated = self.store.update(id, update)?;
                tracing::debug!("handler: GetDatabase branch entered");
                return Ok(serde_json::to_value(updated)?);
            }
            Op::GetConnection => {
                tracing::debug!("handler: GetConnection entered");
                let id = parse_args!(args, u64);
                let conn = self
                    .store
                    .get(id)?
                    .ok_or_else(|| SqlEditorError::Other("Connection not found".to_string()))?;
                tracing::debug!("handler: UpdateDatabase branch entered");
                return Ok(serde_json::to_value(conn)?);
            }
            _ => {
                tracing::debug!("handler: ConnectionStore _ (unsupported) entered");
                return Ok(serde_json::to_value("unsupported")?);
            }
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
                Ok(serde_json::to_value(self.store.list())?)
            }
            Op::AddTable => {
                tracing::debug!("handler: AddTable entered");
                self.store.add(serde_json::from_str(&args[0])?);
                Ok(serde_json::to_value(true)?)
            }
            Op::DeleteTable => {
                tracing::debug!("handler: DeleteTable entered");
                Ok(serde_json::to_value(self.store.delete(&args[0]))?)
            }
            _ => {
                tracing::debug!("handler: TableStore _ (unsupported) entered");
                Ok(serde_json::to_value("unsupported")?)
            }
        }
    }
}
