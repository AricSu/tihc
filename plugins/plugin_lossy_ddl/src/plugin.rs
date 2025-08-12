//! Plugin implementation for Lossy DDL Detection

use std::sync::Arc;

use anyhow::{Context, Result};
use microkernel::plugin_api::traits::{Plugin, PluginContext};
use serde_json::Value;
use tracing::info;
use sqlparser::ast::{Statement, ObjectName};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

use crate::{precheck_sql_with_collation, AnalysisResult};

/// Handler for DDL analysis operations
#[derive(Debug, Clone, Default)]
pub struct DDLAnalysisHandler;

impl DDLAnalysisHandler {
    /// Create a new handler instance
    pub fn new() -> Self {
        Self
    }
    
    /// Analyze SQL for lossy operations
    pub fn analyze_sql(&self, sql: &str, collation_enabled: bool) -> AnalysisResult {
        let processed_sql = match self.preprocess_sql(sql) {
            Ok(processed) => processed,
            Err(e) => {
                return AnalysisResult {
                    lossy_status: crate::types::LossyStatus::Unknown,
                    risk_level: crate::types::RiskLevel::High,
                    warnings: vec![format!("SQL preprocessing error: {}", e)],
                    error: Some(e.to_string()),
                };
            }
        };
        
        precheck_sql_with_collation(&processed_sql, collation_enabled)
    }
    
    /// 验证SQL：必须包含CREATE DATABASE、CREATE TABLE、ALTER TABLE且数据库名一致
    fn preprocess_sql(&self, sql: &str) -> Result<String, crate::error::DDLError> {
        let dialect = MySqlDialect {};
        let statements = Parser::parse_sql(&dialect, sql.trim())
            .map_err(|e| crate::error::DDLError::InvalidInput(format!("SQL parsing failed: {}", e)))?;
        
        if statements.is_empty() {
            return Err(crate::error::DDLError::InvalidInput("No valid statements found".to_string()));
        }
        
        let mut create_db_name: Option<String> = None;
        let mut create_table_db: Option<String> = None;
        let mut alter_table_db: Option<String> = None;
        let mut has_create_db = false;
        let mut has_create_table = false;
        let mut has_alter_table = false;
        
        // 验证所有必需的语句和数据库名一致性
        for stmt in &statements {
            match stmt {
                Statement::CreateDatabase { db_name, .. } => {
                    has_create_db = true;
                    if let Some(sqlparser::ast::ObjectNamePart::Identifier(ident)) = db_name.0.first() {
                        create_db_name = Some(ident.value.clone());
                    }
                }
                Statement::CreateTable(create_table) => {
                    has_create_table = true;
                    create_table_db = self.extract_db_name_from_table(&create_table.name)?;
                }
                Statement::AlterTable { name, .. } => {
                    has_alter_table = true;
                    alter_table_db = self.extract_db_name_from_table(name)?;
                }
                _ => {} // 忽略其他语句
            }
        }
        
        // 检查必需的语句
        if !has_create_db {
            return Err(crate::error::DDLError::InvalidInput("Missing CREATE DATABASE statement".to_string()));
        }
        if !has_create_table {
            return Err(crate::error::DDLError::InvalidInput("Missing CREATE TABLE statement".to_string()));
        }
        if !has_alter_table {
            return Err(crate::error::DDLError::InvalidInput("Missing ALTER TABLE statement".to_string()));
        }
        
        // 检查数据库名一致性
        let expected_db = create_db_name.ok_or_else(|| 
            crate::error::DDLError::InvalidInput("CREATE DATABASE statement must specify database name".to_string()))?;
        
        if create_table_db != Some(expected_db.clone()) {
            return Err(crate::error::DDLError::InvalidInput(format!(
                "CREATE TABLE must use database '{}', found: {:?}", 
                expected_db, create_table_db
            )));
        }
        
        if alter_table_db != Some(expected_db.clone()) {
            return Err(crate::error::DDLError::InvalidInput(format!(
                "ALTER TABLE must use database '{}', found: {:?}", 
                expected_db, alter_table_db
            )));
        }
        
        // 验证通过，返回原始SQL
        Ok(sql.to_string())
    }
    
    /// 从表名中提取数据库名（必须有数据库前缀）
    fn extract_db_name_from_table(&self, name: &ObjectName) -> Result<Option<String>, crate::error::DDLError> {
        if name.0.len() < 2 {
            return Err(crate::error::DDLError::InvalidInput(
                "Table name must include database prefix (e.g., 'database.table')".to_string()
            ));
        }
        
        if let sqlparser::ast::ObjectNamePart::Identifier(ident) = &name.0[0] {
            Ok(Some(ident.value.clone()))
        } else {
            Err(crate::error::DDLError::InvalidInput("Invalid database name format".to_string()))
        }
    }
    
    /// Parse command arguments
    fn parse_args(&self, args: &[String]) -> Result<(String, bool), crate::error::DDLError> {
        if args.is_empty() {
            return Err(crate::error::DDLError::InvalidInput("SQL statement is required".to_string()));
        }
        
        let sql = args[0].clone();
        let collation_enabled = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(true);
        
        Ok((sql, collation_enabled))
    }
}

/// Command handler implementation
#[async_trait::async_trait]
impl microkernel::platform::command_registry::CommandHandler for DDLAnalysisHandler {
    async fn handle(&self, args: &[String]) -> Result<Value> {
        let (sql, collation_enabled) = self.parse_args(args)
            .map_err(|e| anyhow::anyhow!("Argument parsing failed: {}", e))?;
        
        let result = self.analyze_sql(&sql, collation_enabled);
        serde_json::to_value(result).with_context(|| "Failed to serialize analysis result")
    }
}

/// Main plugin for lossy DDL detection
#[derive(Debug, Default)]
pub struct LossyDDLPlugin {
    handler: Arc<DDLAnalysisHandler>,
}

impl LossyDDLPlugin {
    pub fn new() -> Self {
        Self {
            handler: Arc::new(DDLAnalysisHandler::new()),
        }
    }
    
    pub fn handler(&self) -> Arc<DDLAnalysisHandler> {
        Arc::clone(&self.handler)
    }
}

impl Plugin for LossyDDLPlugin {
    fn name(&self) -> &str {
        "lossy_ddl"
    }
    
    fn register(&mut self, ctx: &mut PluginContext) {
        if let Some(registry) = ctx.command_registry.as_mut() {
            registry.register("ddl-precheck", Box::new(DDLAnalysisHandler::new()));
            info!("Registered 'ddl-precheck' command handler");
        }
        info!("LossyDDLPlugin registration completed");
    }
}
