use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Profile error: {0}")]
    Profile(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("SQL error: {0}")]
    Sql(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Time error: {0}")]
    Time(String),
}

pub type Result<T> = std::result::Result<T, UtilsError>;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Failed to send USR1 signal: {0}")]
    SignalError(String),
    #[error("Failed to get log path: {0}")]
    LogPathError(String),
    #[error("Failed to find pprof port in log: {0}")]
    PprofPortError(String),
    #[error("Failed to collect profile: {0}")]
    CollectError(String),
}

#[derive(Debug, Error)]
pub enum DataFetchError {
    #[error("Failed to fetch data: {0}")]
    FetchError(String),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Failed to save data: {0}")]
    SaveError(String),
}

/// 自定义错误类型
#[derive(Debug, Error)]
pub enum TestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Process not found: {0}")]
    ProcessNotFound(String),
    #[error("Log pattern not found")]
    LogPatternNotFound,
    #[error("Signal send failed: {0}")]
    SignalError(#[from] nix::Error),
    #[error("Port not found")]
    PortNotFound,
}
