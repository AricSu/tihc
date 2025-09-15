use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported database engines
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseEngine {
    MySQL,
    PostgreSQL,
    SQLite,
    TiDB,
    Oracle,
    MSSQL,
}

impl DatabaseEngine {
    pub fn from_string(engine: &str) -> Self {
        match engine.to_lowercase().as_str() {
            "mysql" => DatabaseEngine::MySQL,
            "postgresql" | "postgres" => DatabaseEngine::PostgreSQL,
            "sqlite" => DatabaseEngine::SQLite,
            "tidb" => DatabaseEngine::TiDB,
            "oracle" => DatabaseEngine::Oracle,
            "mssql" | "sqlserver" => DatabaseEngine::MSSQL,
            _ => DatabaseEngine::MySQL, // Default fallback
        }
    }

    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseEngine::MySQL | DatabaseEngine::TiDB => 3306,
            DatabaseEngine::PostgreSQL => 5432,
            DatabaseEngine::SQLite => 0, // SQLite doesn't use ports
            DatabaseEngine::Oracle => 1521,
            DatabaseEngine::MSSQL => 1433,
        }
    }

    pub fn supports_tls(&self) -> bool {
        match self {
            DatabaseEngine::SQLite => false,
            _ => true,
        }
    }
}

/// Value object for connection identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectionId(u64);

impl ConnectionId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// TLS configuration for secure connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub ca_cert_path: Option<String>,
    pub client_cert_path: Option<String>,
    pub client_key_path: Option<String>,
    pub verify_server_cert: bool,
}

impl TlsConfig {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ca_cert_path: None,
            client_cert_path: None,
            client_key_path: None,
            verify_server_cert: true,
        }
    }

    pub fn enabled_with_ca(ca_cert_path: String) -> Self {
        Self {
            enabled: true,
            ca_cert_path: Some(ca_cert_path),
            client_cert_path: None,
            client_key_path: None,
            verify_server_cert: true,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.enabled {
            if let Some(ref ca_path) = self.ca_cert_path {
                if ca_path.is_empty() {
                    return Err(anyhow::anyhow!(
                        "CA certificate path cannot be empty when TLS is enabled"
                    ));
                }
            }

            // If client cert is provided, key must also be provided
            match (&self.client_cert_path, &self.client_key_path) {
                (Some(_), None) => {
                    return Err(anyhow::anyhow!(
                        "Client key path is required when client certificate is provided"
                    ))
                }
                (None, Some(_)) => {
                    return Err(anyhow::anyhow!(
                        "Client certificate path is required when client key is provided"
                    ))
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Database connection aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub id: ConnectionId,
    pub name: String,
    pub engine: DatabaseEngine,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>, // Encrypted in real implementation
    pub database: Option<String>,
    pub tls_config: TlsConfig,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub connection_metadata: HashMap<String, String>,
}

impl DatabaseConnection {
    pub fn new(
        id: ConnectionId,
        name: String,
        engine: DatabaseEngine,
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        database: Option<String>,
        tls_config: TlsConfig,
    ) -> anyhow::Result<Self> {
        // Validate TLS configuration
        tls_config.validate()?;

        // Validate basic connection parameters
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Connection name cannot be empty"));
        }

        if host.trim().is_empty() {
            return Err(anyhow::anyhow!("Host cannot be empty"));
        }

        if username.trim().is_empty() {
            return Err(anyhow::anyhow!("Username cannot be empty"));
        }

        if port == 0 && engine != DatabaseEngine::SQLite {
            return Err(anyhow::anyhow!(
                "Port cannot be zero for {} connections",
                engine.to_string()
            ));
        }

        let now = chrono::Utc::now();
        Ok(Self {
            id,
            name: name.trim().to_string(),
            engine,
            host: host.trim().to_string(),
            port,
            username: username.trim().to_string(),
            password,
            database,
            tls_config,
            is_active: true,
            created_at: now,
            updated_at: now,
            connection_metadata: HashMap::new(),
        })
    }

    /// Updates connection details
    pub fn update(
        &mut self,
        name: Option<String>,
        host: Option<String>,
        port: Option<u16>,
        username: Option<String>,
        password: Option<String>,
        database: Option<String>,
        tls_config: Option<TlsConfig>,
    ) -> anyhow::Result<()> {
        if let Some(name) = name {
            if name.trim().is_empty() {
                return Err(anyhow::anyhow!("Connection name cannot be empty"));
            }
            self.name = name.trim().to_string();
        }

        if let Some(host) = host {
            if host.trim().is_empty() {
                return Err(anyhow::anyhow!("Host cannot be empty"));
            }
            self.host = host.trim().to_string();
        }

        if let Some(port) = port {
            if port == 0 && self.engine != DatabaseEngine::SQLite {
                return Err(anyhow::anyhow!(
                    "Port cannot be zero for {} connections",
                    self.engine.to_string()
                ));
            }
            self.port = port;
        }

        if let Some(username) = username {
            if username.trim().is_empty() {
                return Err(anyhow::anyhow!("Username cannot be empty"));
            }
            self.username = username.trim().to_string();
        }

        if let Some(password) = password {
            self.password = Some(password);
        }

        if let Some(database) = database {
            self.database = Some(database);
        }

        if let Some(tls_config) = tls_config {
            tls_config.validate()?;
            self.tls_config = tls_config;
        }

        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Deactivates the connection
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = chrono::Utc::now();
    }

    /// Activates the connection
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now();
    }

    /// Gets the connection string (without password for logging)
    pub fn get_display_connection_string(&self) -> String {
        match self.engine {
            DatabaseEngine::SQLite => {
                format!("sqlite://{}", self.database.as_deref().unwrap_or(""))
            }
            _ => {
                let db_part = self
                    .database
                    .as_ref()
                    .map(|d| format!("/{}", d))
                    .unwrap_or_default();
                format!(
                    "{}://{}@{}:{}{}",
                    self.engine.to_string().to_lowercase(),
                    self.username,
                    self.host,
                    self.port,
                    db_part
                )
            }
        }
    }

    /// Gets the full connection string with password (for actual connections)
    pub fn get_connection_string(&self) -> String {
        match self.engine {
            DatabaseEngine::SQLite => {
                format!("sqlite://{}", self.database.as_deref().unwrap_or(""))
            }
            _ => {
                let password_part = self
                    .password
                    .as_ref()
                    .map(|p| format!(":{}", p))
                    .unwrap_or_default();
                let db_part = self
                    .database
                    .as_ref()
                    .map(|d| format!("/{}", d))
                    .unwrap_or_default();
                format!(
                    "{}://{}{password_part}@{}:{}{}",
                    self.engine.to_string().to_lowercase(),
                    self.username,
                    self.host,
                    self.port,
                    db_part
                )
            }
        }
    }

    /// Adds metadata to the connection
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.connection_metadata.insert(key, value);
        self.updated_at = chrono::Utc::now();
    }

    /// Removes metadata from the connection
    pub fn remove_metadata(&mut self, key: &str) {
        self.connection_metadata.remove(key);
        self.updated_at = chrono::Utc::now();
    }
}

impl std::fmt::Display for DatabaseEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseEngine::MySQL => write!(f, "MySQL"),
            DatabaseEngine::PostgreSQL => write!(f, "PostgreSQL"),
            DatabaseEngine::SQLite => write!(f, "SQLite"),
            DatabaseEngine::TiDB => write!(f, "TiDB"),
            DatabaseEngine::Oracle => write!(f, "Oracle"),
            DatabaseEngine::MSSQL => write!(f, "MSSQL"),
        }
    }
}

/// Request for creating database connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database: Option<String>,
    pub use_tls: bool,
    pub ca_cert_path: Option<String>,
}

impl CreateConnectionRequest {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("Connection name is required"));
        }

        if self.engine.trim().is_empty() {
            return Err(anyhow::anyhow!("Database engine is required"));
        }

        if self.host.trim().is_empty() {
            return Err(anyhow::anyhow!("Host is required"));
        }

        if self.username.trim().is_empty() {
            return Err(anyhow::anyhow!("Username is required"));
        }

        let engine = DatabaseEngine::from_string(&self.engine);
        if self.port == 0 && engine != DatabaseEngine::SQLite {
            return Err(anyhow::anyhow!(
                "Port is required for {} connections",
                engine
            ));
        }

        // TLS validation
        if self.use_tls {
            if !engine.supports_tls() {
                return Err(anyhow::anyhow!("{} does not support TLS", engine));
            }
        }

        Ok(())
    }

    pub fn to_connection(&self, id: ConnectionId) -> anyhow::Result<DatabaseConnection> {
        self.validate()?;

        let engine = DatabaseEngine::from_string(&self.engine);
        let port = if self.port == 0 {
            engine.default_port()
        } else {
            self.port
        };

        let tls_config = if self.use_tls {
            if let Some(ref ca_path) = self.ca_cert_path {
                TlsConfig::enabled_with_ca(ca_path.clone())
            } else {
                TlsConfig {
                    enabled: true,
                    ca_cert_path: None,
                    client_cert_path: None,
                    client_key_path: None,
                    verify_server_cert: true,
                }
            }
        } else {
            TlsConfig::disabled()
        };

        DatabaseConnection::new(
            id,
            self.name.clone(),
            engine,
            self.host.clone(),
            port,
            self.username.clone(),
            self.password.clone(),
            self.database.clone(),
            tls_config,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_engine_from_string() {
        assert_eq!(DatabaseEngine::from_string("mysql"), DatabaseEngine::MySQL);
        assert_eq!(DatabaseEngine::from_string("MySQL"), DatabaseEngine::MySQL);
        assert_eq!(
            DatabaseEngine::from_string("postgresql"),
            DatabaseEngine::PostgreSQL
        );
        assert_eq!(
            DatabaseEngine::from_string("postgres"),
            DatabaseEngine::PostgreSQL
        );
        assert_eq!(
            DatabaseEngine::from_string("sqlite"),
            DatabaseEngine::SQLite
        );
        assert_eq!(DatabaseEngine::from_string("tidb"), DatabaseEngine::TiDB);
        assert_eq!(
            DatabaseEngine::from_string("unknown"),
            DatabaseEngine::MySQL
        );
    }

    #[test]
    fn test_database_engine_default_port() {
        assert_eq!(DatabaseEngine::MySQL.default_port(), 3306);
        assert_eq!(DatabaseEngine::PostgreSQL.default_port(), 5432);
        assert_eq!(DatabaseEngine::SQLite.default_port(), 0);
        assert_eq!(DatabaseEngine::TiDB.default_port(), 3306);
    }

    #[test]
    fn test_connection_id() {
        let id = ConnectionId::new(123);
        assert_eq!(id.value(), 123);
        assert_eq!(format!("{}", id), "123");
    }

    #[test]
    fn test_tls_config_validation() {
        let disabled = TlsConfig::disabled();
        assert!(disabled.validate().is_ok());

        let enabled = TlsConfig::enabled_with_ca("path/to/ca.pem".to_string());
        assert!(enabled.validate().is_ok());

        let invalid = TlsConfig {
            enabled: true,
            ca_cert_path: Some("".to_string()),
            client_cert_path: None,
            client_key_path: None,
            verify_server_cert: true,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_database_connection_creation() {
        let id = ConnectionId::new(1);
        let tls_config = TlsConfig::disabled();

        let connection = DatabaseConnection::new(
            id,
            "Test Connection".to_string(),
            DatabaseEngine::MySQL,
            "localhost".to_string(),
            3306,
            "user".to_string(),
            Some("password".to_string()),
            Some("testdb".to_string()),
            tls_config,
        );

        assert!(connection.is_ok());
        let conn = connection.unwrap();
        assert_eq!(conn.name, "Test Connection");
        assert_eq!(conn.engine, DatabaseEngine::MySQL);
    }

    #[test]
    fn test_database_connection_validation() {
        let id = ConnectionId::new(1);
        let tls_config = TlsConfig::disabled();

        // Empty name should fail
        let result = DatabaseConnection::new(
            id,
            "".to_string(),
            DatabaseEngine::MySQL,
            "localhost".to_string(),
            3306,
            "user".to_string(),
            None,
            None,
            tls_config.clone(),
        );
        assert!(result.is_err());

        // Empty host should fail
        let result = DatabaseConnection::new(
            id,
            "Test".to_string(),
            DatabaseEngine::MySQL,
            "".to_string(),
            3306,
            "user".to_string(),
            None,
            None,
            tls_config.clone(),
        );
        assert!(result.is_err());

        // Zero port should fail for non-SQLite
        let result = DatabaseConnection::new(
            id,
            "Test".to_string(),
            DatabaseEngine::MySQL,
            "localhost".to_string(),
            0,
            "user".to_string(),
            None,
            None,
            tls_config,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_connection_string_generation() {
        let id = ConnectionId::new(1);
        let tls_config = TlsConfig::disabled();

        let connection = DatabaseConnection::new(
            id,
            "Test".to_string(),
            DatabaseEngine::MySQL,
            "localhost".to_string(),
            3306,
            "user".to_string(),
            Some("password".to_string()),
            Some("testdb".to_string()),
            tls_config,
        )
        .unwrap();

        let display_str = connection.get_display_connection_string();
        assert_eq!(display_str, "mysql://user@localhost:3306/testdb");

        let conn_str = connection.get_connection_string();
        assert_eq!(conn_str, "mysql://user:password@localhost:3306/testdb");
    }

    #[test]
    fn test_create_connection_request_validation() {
        let valid_request = CreateConnectionRequest {
            name: "Test Connection".to_string(),
            engine: "mysql".to_string(),
            host: "localhost".to_string(),
            port: 3306,
            username: "user".to_string(),
            password: Some("pass".to_string()),
            database: Some("testdb".to_string()),
            use_tls: false,
            ca_cert_path: None,
        };
        assert!(valid_request.validate().is_ok());

        // Test empty name
        let mut invalid_request = valid_request.clone();
        invalid_request.name = "".to_string();
        assert!(invalid_request.validate().is_err());

        // Test empty engine
        let mut invalid_request = valid_request.clone();
        invalid_request.engine = "".to_string();
        assert!(invalid_request.validate().is_err());
    }
}
