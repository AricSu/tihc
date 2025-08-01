use serde::Serialize;

/// Query result for SQL execution.
#[derive(Serialize, Debug, Default)]
pub struct SqlQueryResult {
    /// Column names of the result set.
    pub columns: Vec<String>,
    /// Column type names of the result set.
    pub column_types: Vec<String>,
    /// Row data, each row is a vector of JSON values.
    pub rows: Vec<Vec<serde_json::Value>>,
    /// Informational or warning messages from the database engine.
    pub messages: Vec<SqlMessage>,
}

/// Message returned by the database engine (e.g., warning, notice).
#[derive(Serialize, Debug, Default)]
pub struct SqlMessage {
    /// Message level (e.g., WARNING, INFO).
    pub level: String,
    /// Message content.
    pub content: String,
}
