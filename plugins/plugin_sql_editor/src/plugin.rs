//! Plugin trait implementation and registration for SQL Editor.
use crate::application::handler::{Command, Op};
use crate::infrastructure::connection_store::ConnectionStore;
use core::plugin_api::traits::Plugin;
use std::sync::Arc;
// 假设已存在 DatabaseStore 实现
use crate::infrastructure::database_store::{DatabaseStore};

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

impl Plugin for SqlEditorPlugin {
    fn name(&self) -> &str {
        "sql_editor"
    }
    fn register(&mut self, ctx: &mut core::plugin_api::traits::PluginContext) {
        let conn_store = Arc::new(ConnectionStore::new());
        let table_store = Arc::new(crate::infrastructure::table_store::TableStore::new());
        self.add_db_pool(DbPoolType::Dummy);
        // Use DatabaseStore::new with a dummy pool (e.g., None or a dummy type)
        let dummy_db_store = Arc::new(DatabaseStore::new(crate::infrastructure::database_store::DbPool::Dummy));
        if let Some(reg) = ctx.command_registry.as_mut() {
            reg.register(
                "editor-connections-get",
                Box::new(Command {
                    store: Arc::clone(&conn_store),
                    op: Op::GetConnection,
                }),
            );
            // Connection commands
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
            // Table commands
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

            // 所有数据库相关命令均用 Dummy 占位类型注册，实际分发由 handler.rs 动态完成
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
        // 后台任务由平台统一调度，传入 shutdown_rx
        if let Some(shutdown_rx) = ctx.shutdown_rx.take() {
            SqlEditorPlugin::start_background_task(shutdown_rx);
        }
    }
}
