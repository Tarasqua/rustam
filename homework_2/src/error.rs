//! Error types for the Smart Home library.

use thiserror::Error;

/// The unified error type for all Smart Home operations.
///
/// Covers I/O failures, protocol violations, missing data,
/// timeouts, and configuration problems.
#[derive(Debug, Error)]
pub enum SmartHomeError {
    /// Wraps a standard I/O error (e.g. connection refused, broken pipe).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The remote device sent a response that could not be parsed.
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// No temperature data has been received yet.
    #[error("No data available yet")]
    NoData,

    /// The operation did not complete within the allowed time.
    #[error("Connection timeout")]
    Timeout,

    /// The configuration file is missing or malformed.
    #[error("Config error: {0}")]
    Config(String),
}
