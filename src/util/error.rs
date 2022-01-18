use thiserror::Error;

/// An error originating from the TiHC.
#[derive(Debug, Error)]
pub enum Error {
    /// Wraps a `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
