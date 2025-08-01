use crate::domain::database::DatabaseConnection;
use crate::domain::error::SqlEditorError;
use crate::domain::sql::SqlQueryResult;
use crate::infrastructure::database_store::DatabaseStore;

/// Application service for SQL editor use cases.
pub struct SqlEditorService;

impl SqlEditorService {
    /// Execute SQL by connection_id and SQL string.
    pub fn execute_sql(connection_id: u64, sql: &str) -> Result<SqlQueryResult, SqlEditorError> {
        // TODO: In the future, lookup connection info by id from repository.
        // For now, use hardcoded connection info (tidb/mysql).
        let conn = DatabaseConnection {
            id: connection_id,
            name: "default".to_string(),
            engine: "tidb".to_string(),
            host: "127.0.0.1".to_string(),
            port: 4000,
            username: "root".to_string(),
            password: None,
            database: Some("test".to_string()),
            use_tls: false,
            ca_cert_path: None,
            created_at: "".to_string(),
        };
        let pool = DatabaseStore::with_mysql(&conn);
        let handle = tokio::runtime::Handle::current();
        handle.block_on(pool.execute_sql(sql))
    }
}
