//! Plugin trait implementation and registration for SQL Editor.
use crate::application::handler::{Command, ExecuteSqlCommand, Op, TestConnectionCommand};
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use core::plugin_api::traits::Plugin;
use std::sync::Arc;
// 假设已存在 DatabaseStore 实现
use crate::infrastructure::database_store::{DatabaseStore, DbPool};

pub struct SqlEditorPlugin;

impl Plugin for SqlEditorPlugin {
    fn name(&self) -> &str {
        "sql_editor"
    }
    fn register(&mut self, ctx: &mut core::plugin_api::traits::PluginContext) {
        let conn_store = Arc::new(ConnectionStore::new());
        let table_store = Arc::new(TableStore::new());
        if let Some(reg) = ctx.command_registry.as_mut() {
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
            reg.register(
                "editor-tables-add-column",
                Box::new(Command {
                    store: Arc::clone(&table_store),
                    op: Op::AddColumn,
                }),
            );
            reg.register(
                "editor-tables-delete-column",
                Box::new(Command {
                    store: Arc::clone(&table_store),
                    op: Op::DeleteColumn,
                }),
            );
            // Database/schema commands
            let dummy_db_store = Arc::new(DatabaseStore {
                pool: DbPool::Dummy,
            });
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
            // SQL execute
            reg.register("editor-sql-execute", Box::new(ExecuteSqlCommand {}));
        }
    }
}
