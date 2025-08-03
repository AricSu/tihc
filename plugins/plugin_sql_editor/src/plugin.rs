//! Plugin trait implementation and registration for SQL Editor.
use crate::application::handler::{Command, Op};
use crate::infrastructure::connection_store::ConnectionStore;
use microkernel::plugin_api::traits::Plugin;
use std::sync::Arc;
// 假设已存在 DatabaseStore 实现
use crate::infrastructure::database_store::DatabaseStore;

pub enum DbPoolType {
    MySql(Arc<sqlx::MySqlPool>),
    Dummy,
    // ...更多类型
}

pub struct SqlEditorPlugin {
    db_pools: Vec<DbPoolType>,
    // 可扩展：缓存、文件句柄等
}

impl SqlEditorPlugin {
    pub fn new() -> Self {
        Self {
            db_pools: Vec::new(),
        }
    }
    /// 启动后台任务，订阅 shutdown 信号
    pub fn start_background_task(shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        tokio::spawn(async move {
            let mut shutdown_rx = shutdown_rx;
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        tracing::info!("[SqlEditorPlugin] Background task received shutdown signal, exiting.");
                        break;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                        // ...实际后台任务逻辑...
                    }
                }
            }
            tracing::info!("[SqlEditorPlugin] Background task cleanup done.");
        });
    }
    /// 动态添加数据库连接池
    pub fn add_db_pool(&mut self, pool: DbPoolType) {
        self.db_pools.push(pool);
    }
    // 可扩展：缓存、文件句柄等资源初始化方法
}

/// Implements the Plugin trait for SqlEditorPlugin.
/// Registers all SQL Editor commands and services with the plugin context.
impl Plugin for SqlEditorPlugin {
    /// Returns the plugin name for registration and discovery.
    fn name(&self) -> &str {
        "sql_editor"
    }
    /// Registers all command handlers and services for the SQL Editor plugin.
    ///
    /// This includes connection, table, and database commands, as well as background tasks.
    fn register(&mut self, ctx: &mut microkernel::plugin_api::traits::PluginContext) {
        // Register in-memory connection store for connection-related commands.
        let conn_store = Arc::new(ConnectionStore::new());
        // Register in-memory table store for table-related commands.
        let table_store = Arc::new(crate::infrastructure::table_store::TableStore::new(Arc::clone(&conn_store)));
        // Register a dummy database pool for database-related commands.
        self.add_db_pool(DbPoolType::Dummy);
        let dummy_db_store = Arc::new(DatabaseStore::new_dummy(Arc::clone(&conn_store)));
        if let Some(reg) = ctx.command_registry.as_mut() {
            // Register connection-related commands.
            reg.register(
                "editor-connections-get",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::GetConnection,
                }),
            );
            reg.register(
                "editor-connections-list",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::ListConnection,
                }),
            );
            reg.register(
                "editor-connections-create",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::AddConnection,
                }),
            );
            reg.register(
                "editor-connections-delete",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::DeleteConnection,
                }),
            );
            reg.register(
                "editor-connections-test",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::TestConnection,
                }),
            );
            reg.register(
                "editor-connections-update",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::UpdateConnection,
                }),
            );
            // Register table-related commands.
            reg.register(
                "editor-tables-list",
                Box::new(Command {
                    store: Arc::clone(&table_store),
                    op: Op::ListTable,
                }),
            );
            reg.register(
                "editor-tables-add",
                Box::new(Command {
                    store: Arc::clone(&table_store),
                    op: Op::AddTable,
                }),
            );
            reg.register(
                "editor-tables-delete",
                Box::new(Command {
                    store: Arc::clone(&table_store),
                    op: Op::DeleteTable,
                }),
            );

            // Register database-related commands with dummy store; actual dispatch is handled in handler.rs.
            reg.register(
                "editor-sql-execute",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::ExecuteSql,
                }),
            );
            reg.register(
                "editor-databases-list",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::ListDatabase,
                }),
            );
            reg.register(
                "editor-databases-add",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::AddDatabase,
                }),
            );
            reg.register(
                "editor-databases-delete",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::DeleteDatabase,
                }),
            );
            reg.register(
                "editor-databases-get",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::GetDatabase,
                }),
            );
            reg.register(
                "editor-databases-update",
                Box::new(Command {
                    store: Arc::clone(&dummy_db_store),
                    op: Op::UpdateDatabase,
                }),
            );
        }
        // Start background task if shutdown signal is provided by the platform.
        if let Some(shutdown_rx) = ctx.shutdown_rx.take() {
            SqlEditorPlugin::start_background_task(shutdown_rx);
        }
    }
}
