//! Error types for G.A.N.E NAV.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuroraError {
    #[error("GNSS error: {0}")]
    Gnss(String),

    #[error("Sensor error: {0}")]
    Sensor(String),

    #[error("Fusion error: {0}")]
    Fusion(String),

    #[error("Integrity violation: {0}")]
    Integrity(String),

    #[error("Correction error: {0}")]
    Correction(String),

    #[error("Continuity error: {0}")]
    Continuity(String),

    #[error("Map error: {0}")]
    Map(String),

    #[error("Route error: {0}")]
    Route(String),

    #[error("Synchronization error: {0}")]
    Sync(String),

    #[error("Telemetry error: {0}")]
    Telemetry(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Source excluded: {source_name} — {reason}")]
    SourceExcluded { source_name: String, reason: String },

    #[error("No valid solution: {0}")]
    NoSolution(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type AuroraResult<T> = Result<T, AuroraError>;
