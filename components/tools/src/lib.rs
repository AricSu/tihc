pub mod column_replacer;
use sqlparser::ast::Statement;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

use crate::column_replacer::ColumnReplacer;

pub fn parse_sql(sql: &str) -> Result<Vec<Statement>, String> {
    let parse_result = Parser::parse_sql(&MySqlDialect {}, sql);
    match parse_result {
        Ok(statements) => Ok(statements),
        Err(e) => Err(format!("SQL parse error: {}", e)),
    }
}

pub fn replace_all_column_names(sql: &str) -> Result<String, String> {
    // 预处理 SQL 文本，将 `...` 替换为合法占位符
    let preprocessed_sql = sql.replace("...", "?");

    // 解析 SQL 文本
    let dialect = MySqlDialect {};
    let mut statements =
        Parser::parse_sql(&dialect, &preprocessed_sql).map_err(|e| e.to_string())?;

    // 替换列名
    let mut column_replacer = ColumnReplacer::new();
    column_replacer.apply(&mut statements);

    // 将替换后的 SQL 文本转换回字符串
    let replaced_sql = statements
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<_>>()
        .join("; ")
        .replace("...", "?");

    Ok(replaced_sql) // 返回修改后的 SQL 字符串
}
