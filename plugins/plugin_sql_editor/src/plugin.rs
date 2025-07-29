//! Plugin trait implementation and registration for SQL Editor.
use core::plugin_api::traits::{Plugin, PluginContext};
use std::sync::Arc;
use crate::infrastructure::{connection_store::ConnectionStore, table_store::TableStore};
use crate::application::handler::{Command, ConnectionCommand, ConnectionOp, DeleteCommand, AddColumnCommand, ExecuteSqlCommand, AddTableCommand, DeleteTableCommand, GetConnectionCommand, UpdateConnectionCommand};

pub struct SqlEditorPlugin;

impl Plugin for SqlEditorPlugin {
    fn name(&self) -> &str {
        "sql_editor"
    }
    fn register(&mut self, ctx: &mut PluginContext) {
        let conn_store = Arc::new(ConnectionStore::new());
        let table_store = Arc::new(TableStore::new());
        if let Some(reg) = ctx.command_registry.as_mut() {
            reg.register("editor-connections-list", Box::new(ConnectionCommand {
                store: Arc::clone(&conn_store),
                op: ConnectionOp::List,
            }));
            reg.register("editor-connections-create", Box::new(ConnectionCommand {
                store: Arc::clone(&conn_store),
                op: ConnectionOp::Create,
            }));
            reg.register("editor-connections-delete", Box::new(DeleteCommand { store: Arc::clone(&conn_store) }));
            reg.register("editor-connections-get", Box::new(GetConnectionCommand { store: Arc::clone(&conn_store) }));
            reg.register("editor-connections-update", Box::new(UpdateConnectionCommand { store: Arc::clone(&conn_store) }));
            reg.register("editor-tables-list", Box::new(Command { store: Arc::clone(&table_store) }));
            reg.register("editor-tables-add", Box::new(AddTableCommand { store: Arc::clone(&table_store) }));
            reg.register("editor-tables-delete", Box::new(DeleteTableCommand { store: Arc::clone(&table_store) }));
            reg.register("editor-tables-add-column", Box::new(AddColumnCommand { store: Arc::clone(&table_store) }));
            reg.register("editor-tables-delete-column", Box::new(DeleteCommand { store: Arc::clone(&table_store) }));
            reg.register("editor-sql-execute", Box::new(ExecuteSqlCommand));
        }
    }
}
