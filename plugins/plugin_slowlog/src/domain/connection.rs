#[derive(Clone, Debug, serde::Deserialize, Default)]
pub struct Connection {
    pub id: u64,
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

impl Connection {
    /// Returns a MySQL connection URL string for this connection
    pub fn mysql_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.username,
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port
        )
    }
}
