use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a database column type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnType {
    #[serde(rename = "varchar")]
    Varchar(Option<u32>), // size parameter
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "bigint")]
    BigInt,
    #[serde(rename = "decimal")]
    Decimal(Option<(u8, u8)>), // precision, scale
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "datetime")]
    DateTime,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "custom")]
    Custom(String), // for any other type
}

impl ColumnType {
    /// Creates a ColumnType from a string representation
    pub fn from_string(type_str: &str) -> Self {
        match type_str.to_lowercase().as_str() {
            "varchar" => ColumnType::Varchar(None),
            "int" => ColumnType::Int,
            "bigint" => ColumnType::BigInt,
            "decimal" => ColumnType::Decimal(None),
            "text" => ColumnType::Text,
            "datetime" => ColumnType::DateTime,
            "timestamp" => ColumnType::Timestamp,
            "boolean" | "bool" => ColumnType::Boolean,
            "json" => ColumnType::Json,
            _ => ColumnType::Custom(type_str.to_string()),
        }
    }

    /// Converts the ColumnType to its string representation
    pub fn to_string(&self) -> String {
        match self {
            ColumnType::Varchar(Some(size)) => format!("varchar({})", size),
            ColumnType::Varchar(None) => "varchar".to_string(),
            ColumnType::Int => "int".to_string(),
            ColumnType::BigInt => "bigint".to_string(),
            ColumnType::Decimal(Some((precision, scale))) => {
                format!("decimal({},{})", precision, scale)
            }
            ColumnType::Decimal(None) => "decimal".to_string(),
            ColumnType::Text => "text".to_string(),
            ColumnType::DateTime => "datetime".to_string(),
            ColumnType::Timestamp => "timestamp".to_string(),
            ColumnType::Boolean => "boolean".to_string(),
            ColumnType::Json => "json".to_string(),
            ColumnType::Custom(type_str) => type_str.clone(),
        }
    }
}

/// Represents a database column
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub is_primary_key: bool,
    pub is_auto_increment: bool,
}

impl Column {
    /// Creates a new column
    pub fn new(name: String, column_type: ColumnType) -> Self {
        Self {
            name,
            column_type,
            nullable: true,
            default_value: None,
            is_primary_key: false,
            is_auto_increment: false,
        }
    }

    /// Creates a new column from string type
    pub fn from_string_type(name: String, type_str: &str) -> Self {
        Self::new(name, ColumnType::from_string(type_str))
    }

    /// Sets the column as non-nullable
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// Sets the column as primary key
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self.nullable = false; // Primary keys cannot be null
        self
    }

    /// Sets the column as auto increment
    pub fn auto_increment(mut self) -> Self {
        self.is_auto_increment = true;
        self
    }

    /// Sets a default value for the column
    pub fn default(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Validates the column definition
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Column name cannot be empty".to_string());
        }

        if self.is_primary_key && self.nullable {
            return Err("Primary key columns cannot be nullable".to_string());
        }

        if self.is_auto_increment
            && !matches!(self.column_type, ColumnType::Int | ColumnType::BigInt)
        {
            return Err("Auto increment is only allowed for integer types".to_string());
        }

        Ok(())
    }
}

/// Represents a database table identifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableId {
    pub database: Option<String>,
    pub name: String,
}

impl TableId {
    pub fn new(name: String) -> Self {
        Self {
            database: None,
            name,
        }
    }

    pub fn with_database(database: String, name: String) -> Self {
        Self {
            database: Some(database),
            name,
        }
    }

    pub fn full_name(&self) -> String {
        match &self.database {
            Some(db) => format!("{}.{}", db, self.name),
            None => self.name.clone(),
        }
    }
}

/// Represents a database table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: TableId,
    pub columns: Vec<Column>,
    pub indexes: Vec<String>, // For now, simple string representation
    pub engine: Option<String>,
    pub charset: Option<String>,
}

impl Table {
    /// Creates a new table
    pub fn new(id: TableId) -> Self {
        Self {
            id,
            columns: Vec::new(),
            indexes: Vec::new(),
            engine: None,
            charset: None,
        }
    }

    /// Adds a column to the table
    pub fn add_column(mut self, column: Column) -> Result<Self, String> {
        column.validate()?;

        // Check for duplicate column names
        if self.columns.iter().any(|c| c.name == column.name) {
            return Err(format!("Column '{}' already exists", column.name));
        }

        self.columns.push(column);
        Ok(self)
    }

    /// Removes a column from the table
    pub fn remove_column(&mut self, column_name: &str) -> Result<Column, String> {
        let index = self
            .columns
            .iter()
            .position(|c| c.name == column_name)
            .ok_or_else(|| format!("Column '{}' not found", column_name))?;

        let removed_column = self.columns.remove(index);
        Ok(removed_column)
    }

    /// Gets a column by name
    pub fn get_column(&self, column_name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == column_name)
    }

    /// Gets all column names
    pub fn column_names(&self) -> Vec<&str> {
        self.columns.iter().map(|c| c.name.as_str()).collect()
    }

    /// Gets primary key columns
    pub fn primary_key_columns(&self) -> Vec<&Column> {
        self.columns.iter().filter(|c| c.is_primary_key).collect()
    }

    /// Validates the table structure
    pub fn validate(&self) -> Result<(), String> {
        if self.id.name.is_empty() {
            return Err("Table name cannot be empty".to_string());
        }

        if self.columns.is_empty() {
            return Err("Table must have at least one column".to_string());
        }

        // Validate each column
        for column in &self.columns {
            column.validate()?;
        }

        // Check for duplicate column names
        let mut column_names = std::collections::HashSet::new();
        for column in &self.columns {
            if !column_names.insert(&column.name) {
                return Err(format!("Duplicate column name: {}", column.name));
            }
        }

        Ok(())
    }
}

/// Request to add a column to a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddColumnRequest {
    pub column_name: String,
    pub column_type: String,
    pub nullable: Option<bool>,
    pub default_value: Option<String>,
}

impl AddColumnRequest {
    pub fn to_column(&self) -> Column {
        let mut column = Column::from_string_type(self.column_name.clone(), &self.column_type);

        if let Some(nullable) = self.nullable {
            column.nullable = nullable;
        }

        if let Some(default_value) = &self.default_value {
            column.default_value = Some(default_value.clone());
        }

        column
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.column_name.is_empty() {
            return Err("Column name cannot be empty".to_string());
        }

        if self.column_type.is_empty() {
            return Err("Column type cannot be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_type_from_string() {
        assert_eq!(
            ColumnType::from_string("varchar"),
            ColumnType::Varchar(None)
        );
        assert_eq!(ColumnType::from_string("int"), ColumnType::Int);
        assert_eq!(
            ColumnType::from_string("custom_type"),
            ColumnType::Custom("custom_type".to_string())
        );
    }

    #[test]
    fn test_column_creation() {
        let column = Column::new("id".to_string(), ColumnType::Int)
            .primary_key()
            .auto_increment();

        assert_eq!(column.name, "id");
        assert_eq!(column.column_type, ColumnType::Int);
        assert!(column.is_primary_key);
        assert!(column.is_auto_increment);
        assert!(!column.nullable);
    }

    #[test]
    fn test_table_creation() {
        let table_id = TableId::new("users".to_string());
        let mut table = Table::new(table_id);

        let id_column = Column::new("id".to_string(), ColumnType::Int)
            .primary_key()
            .auto_increment();

        table = table.add_column(id_column).unwrap();

        assert_eq!(table.id.name, "users");
        assert_eq!(table.columns.len(), 1);
        assert_eq!(table.columns[0].name, "id");
    }

    #[test]
    fn test_add_column_request() {
        let request = AddColumnRequest {
            column_name: "email".to_string(),
            column_type: "varchar".to_string(),
            nullable: Some(false),
            default_value: None,
        };

        let column = request.to_column();
        assert_eq!(column.name, "email");
        assert_eq!(column.column_type, ColumnType::Varchar(None));
        assert!(!column.nullable);
    }

    #[test]
    fn test_table_validation() {
        let table_id = TableId::new("".to_string());
        let table = Table::new(table_id);

        // Empty table name should fail
        assert!(table.validate().is_err());

        // Table without columns should fail
        let table_id = TableId::new("test".to_string());
        let table = Table::new(table_id);
        assert!(table.validate().is_err());
    }
}
