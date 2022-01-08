use thiserror::Error;

/// An error originating from the TiHC.
#[derive(Debug, Error)]
pub enum Error {
    /// Feature is not implemented.
    #[error("Unimplemented feature")]
    Unimplemented,
    /// Wraps a `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Error can be printed as a string chain.
    #[error("{0}")]
    StringError(String),
}
