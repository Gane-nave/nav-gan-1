//! Error classification — severity levels, error categories, and structured error types.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Error severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational — not an error, but notable.
    Info,
    /// Warning — degraded but functional.
    Warning,
    /// Error — operation failed but system continues.
    Error,
    /// Critical — system stability at risk.
    Critical,
    /// Fatal — system must shut down.
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
            Self::Critical => write!(f, "CRITICAL"),
            Self::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Error category for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Network-related errors (connectivity, timeout, DNS).
    Network,
    /// Hardware/sensor errors (GNSS, IMU, camera).
    Hardware,
    /// Data errors (corrupt, missing, invalid format).
    Data,
    /// Configuration errors (invalid settings, missing config).
    Configuration,
    /// Authentication/authorization errors.
    Auth,
    /// Resource errors (out of memory, disk full, quota exceeded).
    Resource,
    /// Internal logic errors (assertions, invariant violations).
    Internal,
    /// External service errors (third-party API failures).
    ExternalService,
    /// User input errors.
    UserInput,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network => write!(f, "Network"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Data => write!(f, "Data"),
            Self::Configuration => write!(f, "Configuration"),
            Self::Auth => write!(f, "Auth"),
            Self::Resource => write!(f, "Resource"),
            Self::Internal => write!(f, "Internal"),
            Self::ExternalService => write!(f, "ExternalService"),
            Self::UserInput => write!(f, "UserInput"),
        }
    }
}

/// Whether an error is transient (retryable) or permanent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorTransience {
    /// Transient — may succeed on retry.
    Transient,
    /// Permanent — will not succeed on retry.
    Permanent,
    /// Unknown — retry cautiously.
    Unknown,
}

/// A classified, structured error with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedError {
    pub code: String,
    pub message: String,
    pub severity: Severity,
    pub category: ErrorCategory,
    pub transience: ErrorTransience,
    pub source_module: String,
    pub context: std::collections::HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ClassifiedError {
    /// Create a new classified error.
    pub fn new(
        code: &str,
        message: &str,
        severity: Severity,
        category: ErrorCategory,
        transience: ErrorTransience,
        source_module: &str,
    ) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            severity,
            category,
            transience,
            source_module: source_module.to_string(),
            context: std::collections::HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Add context key-value pair.
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// Check if the error is retryable.
    pub fn is_retryable(&self) -> bool {
        self.transience == ErrorTransience::Transient
    }

    /// Check if the error is critical or fatal.
    pub fn is_critical(&self) -> bool {
        self.severity >= Severity::Critical
    }
}

impl fmt::Display for ClassifiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}/{}: {} ({})",
            self.severity, self.category, self.code, self.message, self.source_module
        )
    }
}

/// Aurora system error — the top-level error type.
#[derive(Error, Debug)]
pub enum AuroraError {
    #[error("GNSS error: {0}")]
    Gnss(String),

    #[error("Sensor error: {0}")]
    Sensor(String),

    #[error("Fusion error: {0}")]
    Fusion(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl AuroraError {
    /// Classify this error into a structured ClassifiedError.
    pub fn classify(&self, source_module: &str) -> ClassifiedError {
        let (code, severity, category, transience) = match self {
            Self::Gnss(_) => (
                "GNSS_ERR",
                Severity::Error,
                ErrorCategory::Hardware,
                ErrorTransience::Transient,
            ),
            Self::Sensor(_) => (
                "SENSOR_ERR",
                Severity::Warning,
                ErrorCategory::Hardware,
                ErrorTransience::Transient,
            ),
            Self::Fusion(_) => (
                "FUSION_ERR",
                Severity::Error,
                ErrorCategory::Internal,
                ErrorTransience::Permanent,
            ),
            Self::Network(_) => (
                "NET_ERR",
                Severity::Warning,
                ErrorCategory::Network,
                ErrorTransience::Transient,
            ),
            Self::Config(_) => (
                "CONFIG_ERR",
                Severity::Critical,
                ErrorCategory::Configuration,
                ErrorTransience::Permanent,
            ),
            Self::Storage(_) => (
                "STORAGE_ERR",
                Severity::Error,
                ErrorCategory::Resource,
                ErrorTransience::Unknown,
            ),
            Self::Auth(_) => (
                "AUTH_ERR",
                Severity::Error,
                ErrorCategory::Auth,
                ErrorTransience::Permanent,
            ),
            Self::ResourceExhausted(_) => (
                "RESOURCE_ERR",
                Severity::Critical,
                ErrorCategory::Resource,
                ErrorTransience::Transient,
            ),
            Self::Internal(_) => (
                "INTERNAL_ERR",
                Severity::Critical,
                ErrorCategory::Internal,
                ErrorTransience::Permanent,
            ),
            Self::Timeout(_) => (
                "TIMEOUT_ERR",
                Severity::Warning,
                ErrorCategory::Network,
                ErrorTransience::Transient,
            ),
            Self::NotFound(_) => (
                "NOT_FOUND",
                Severity::Warning,
                ErrorCategory::Data,
                ErrorTransience::Permanent,
            ),
            Self::InvalidInput(_) => (
                "INVALID_INPUT",
                Severity::Warning,
                ErrorCategory::UserInput,
                ErrorTransience::Permanent,
            ),
        };

        ClassifiedError::new(
            code,
            &self.to_string(),
            severity,
            category,
            transience,
            source_module,
        )
    }
}

/// Error aggregator — collects and summarizes errors over time.
pub struct ErrorAggregator {
    errors: parking_lot::RwLock<Vec<ClassifiedError>>,
    max_history: usize,
}

impl ErrorAggregator {
    /// Create a new error aggregator.
    pub fn new(max_history: usize) -> Self {
        Self {
            errors: parking_lot::RwLock::new(Vec::new()),
            max_history,
        }
    }

    /// Record an error.
    pub fn record(&self, error: ClassifiedError) {
        let mut errors = self.errors.write();
        errors.push(error);
        while errors.len() > self.max_history {
            errors.remove(0);
        }
    }

    /// Get error count by severity.
    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.errors
            .read()
            .iter()
            .filter(|e| e.severity == severity)
            .count()
    }

    /// Get error count by category.
    pub fn count_by_category(&self, category: ErrorCategory) -> usize {
        self.errors
            .read()
            .iter()
            .filter(|e| e.category == category)
            .count()
    }

    /// Get recent errors (most recent first).
    pub fn recent(&self, limit: usize) -> Vec<ClassifiedError> {
        let errors = self.errors.read();
        errors.iter().rev().take(limit).cloned().collect()
    }

    /// Total error count.
    pub fn total(&self) -> usize {
        self.errors.read().len()
    }

    /// Clear all recorded errors.
    pub fn clear(&self) {
        self.errors.write().clear();
    }

    /// Get error rate per minute for the last N minutes.
    pub fn error_rate(&self, minutes: i64) -> f64 {
        let cutoff = chrono::Utc::now() - chrono::Duration::minutes(minutes);
        let count = self
            .errors
            .read()
            .iter()
            .filter(|e| e.timestamp > cutoff)
            .count();
        count as f64 / minutes as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Info < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
        assert!(Severity::Error < Severity::Critical);
        assert!(Severity::Critical < Severity::Fatal);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Info.to_string(), "INFO");
        assert_eq!(Severity::Critical.to_string(), "CRITICAL");
    }

    #[test]
    fn test_classified_error_creation() {
        let err = ClassifiedError::new(
            "TEST_001",
            "test error",
            Severity::Error,
            ErrorCategory::Network,
            ErrorTransience::Transient,
            "test_module",
        )
        .with_context("url", "https://example.com");

        assert_eq!(err.code, "TEST_001");
        assert!(err.is_retryable());
        assert!(!err.is_critical());
        assert_eq!(
            err.context.get("url"),
            Some(&"https://example.com".to_string())
        );
    }

    #[test]
    fn test_classified_error_critical() {
        let err = ClassifiedError::new(
            "CRIT_001",
            "critical failure",
            Severity::Critical,
            ErrorCategory::Internal,
            ErrorTransience::Permanent,
            "core",
        );
        assert!(err.is_critical());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_gane_error_classify() {
        let err = AuroraError::Network("connection refused".to_string());
        let classified = err.classify("api");
        assert_eq!(classified.code, "NET_ERR");
        assert_eq!(classified.category, ErrorCategory::Network);
        assert_eq!(classified.transience, ErrorTransience::Transient);
        assert_eq!(classified.source_module, "api");
    }

    #[test]
    fn test_gane_error_classify_config() {
        let err = AuroraError::Config("missing key".to_string());
        let classified = err.classify("config");
        assert!(classified.is_critical());
        assert!(!classified.is_retryable());
    }

    #[test]
    fn test_error_aggregator() {
        let agg = ErrorAggregator::new(100);
        for i in 0..5 {
            agg.record(ClassifiedError::new(
                &format!("ERR_{i}"),
                "test",
                Severity::Error,
                ErrorCategory::Network,
                ErrorTransience::Transient,
                "test",
            ));
        }
        assert_eq!(agg.total(), 5);
        assert_eq!(agg.count_by_severity(Severity::Error), 5);
        assert_eq!(agg.count_by_category(ErrorCategory::Network), 5);
    }

    #[test]
    fn test_error_aggregator_max_history() {
        let agg = ErrorAggregator::new(3);
        for i in 0..5 {
            agg.record(ClassifiedError::new(
                &format!("ERR_{i}"),
                "test",
                Severity::Warning,
                ErrorCategory::Data,
                ErrorTransience::Permanent,
                "test",
            ));
        }
        assert_eq!(agg.total(), 3);
        let recent = agg.recent(10);
        assert_eq!(recent.len(), 3);
        // Most recent first
        assert_eq!(recent[0].code, "ERR_4");
    }

    #[test]
    fn test_error_aggregator_clear() {
        let agg = ErrorAggregator::new(100);
        agg.record(ClassifiedError::new(
            "X",
            "x",
            Severity::Info,
            ErrorCategory::Internal,
            ErrorTransience::Unknown,
            "test",
        ));
        assert_eq!(agg.total(), 1);
        agg.clear();
        assert_eq!(agg.total(), 0);
    }

    #[test]
    fn test_error_display() {
        let err = ClassifiedError::new(
            "NET_001",
            "timeout",
            Severity::Warning,
            ErrorCategory::Network,
            ErrorTransience::Transient,
            "api",
        );
        let display = err.to_string();
        assert!(display.contains("WARN"));
        assert!(display.contains("Network"));
        assert!(display.contains("NET_001"));
        assert!(display.contains("timeout"));
    }
}
