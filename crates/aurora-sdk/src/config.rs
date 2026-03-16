//! SDK configuration — settings for the AURORA NAV SDK client,
//! including connection parameters, retry policies, and feature flags.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

// ---------------------------------------------------------------------------
// SDK configuration
// ---------------------------------------------------------------------------

/// Top-level SDK configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    /// Default API endpoint.
    pub default_endpoint: String,
    /// Maximum number of plugins allowed.
    pub max_plugins: usize,
    /// Maximum requests per session before rate limiting.
    pub max_requests_per_session: u64,
    /// Request timeout in milliseconds.
    pub request_timeout_ms: u64,
    /// Retry policy for failed requests.
    pub retry_policy: RetryPolicy,
    /// Feature flags.
    pub features: FeatureFlags,
    /// Logging configuration.
    pub logging: LoggingConfig,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            default_endpoint: "https://api.aurora-nav.io/v1".to_string(),
            max_plugins: 32,
            max_requests_per_session: 100_000,
            request_timeout_ms: 30_000,
            retry_policy: RetryPolicy::default(),
            features: FeatureFlags::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl SdkConfig {
    /// Validate configuration and return errors if invalid.
    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();

        if self.default_endpoint.is_empty() {
            errors.push(ConfigError {
                field: "default_endpoint".into(),
                message: "endpoint cannot be empty".into(),
            });
        }

        if self.max_plugins == 0 {
            errors.push(ConfigError {
                field: "max_plugins".into(),
                message: "must allow at least 1 plugin".into(),
            });
        }

        if self.request_timeout_ms == 0 {
            errors.push(ConfigError {
                field: "request_timeout_ms".into(),
                message: "timeout must be > 0".into(),
            });
        }

        if self.retry_policy.max_retries > 100 {
            errors.push(ConfigError {
                field: "retry_policy.max_retries".into(),
                message: "max retries too high (>100)".into(),
            });
        }

        if self.retry_policy.base_delay_ms == 0 {
            errors.push(ConfigError {
                field: "retry_policy.base_delay_ms".into(),
                message: "base delay must be > 0".into(),
            });
        }

        if errors.is_empty() {
            debug!("SDK config validation passed");
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Create a minimal development configuration.
    pub fn development() -> Self {
        Self {
            default_endpoint: "http://localhost:8080/v1".to_string(),
            max_plugins: 64,
            max_requests_per_session: u64::MAX,
            request_timeout_ms: 60_000,
            retry_policy: RetryPolicy {
                max_retries: 0,
                ..RetryPolicy::default()
            },
            features: FeatureFlags {
                debug_mode: true,
                ..FeatureFlags::default()
            },
            logging: LoggingConfig {
                level: LogLevel::Debug,
                ..LoggingConfig::default()
            },
        }
    }

    /// Create a production configuration.
    pub fn production() -> Self {
        Self {
            default_endpoint: "https://api.aurora-nav.io/v1".to_string(),
            max_plugins: 16,
            max_requests_per_session: 1_000_000,
            request_timeout_ms: 10_000,
            retry_policy: RetryPolicy::default(),
            features: FeatureFlags::default(),
            logging: LoggingConfig {
                level: LogLevel::Warn,
                ..LoggingConfig::default()
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Retry policy
// ---------------------------------------------------------------------------

/// Retry policy for failed requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Base delay between retries in milliseconds.
    pub base_delay_ms: u64,
    /// Maximum delay between retries in milliseconds.
    pub max_delay_ms: u64,
    /// Backoff strategy.
    pub strategy: BackoffStrategy,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
            strategy: BackoffStrategy::ExponentialWithJitter,
        }
    }
}

impl RetryPolicy {
    /// Compute the delay for a given attempt (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        if attempt >= self.max_retries {
            return 0;
        }
        let raw = match self.strategy {
            BackoffStrategy::Fixed => self.base_delay_ms,
            BackoffStrategy::Linear => self.base_delay_ms * (attempt as u64 + 1),
            BackoffStrategy::Exponential => self.base_delay_ms * 2u64.saturating_pow(attempt),
            BackoffStrategy::ExponentialWithJitter => {
                // Deterministic "jitter" based on attempt number for testability.
                let base = self.base_delay_ms * 2u64.saturating_pow(attempt);
                let jitter = (attempt as u64 * 17) % (base.max(1));
                base.saturating_add(jitter)
            }
        };
        raw.min(self.max_delay_ms)
    }

    /// Check if a retry is allowed for the given attempt number.
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_retries
    }
}

/// Backoff strategy for retries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// Fixed delay between retries.
    Fixed,
    /// Linearly increasing delay.
    Linear,
    /// Exponentially increasing delay.
    Exponential,
    /// Exponential with jitter to prevent thundering herd.
    ExponentialWithJitter,
}

// ---------------------------------------------------------------------------
// Feature flags
// ---------------------------------------------------------------------------

/// SDK feature flags for toggling behaviour.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Enable debug mode (extra logging, assertions).
    pub debug_mode: bool,
    /// Enable offline caching.
    pub offline_cache: bool,
    /// Enable telemetry collection.
    pub telemetry: bool,
    /// Enable compression for requests.
    pub compression: bool,
    /// Enable request batching.
    pub batching: bool,
    /// Enable automatic reconnection.
    pub auto_reconnect: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            debug_mode: false,
            offline_cache: true,
            telemetry: true,
            compression: true,
            batching: false,
            auto_reconnect: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------

/// Logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Minimum log level.
    pub level: LogLevel,
    /// Whether to include timestamps.
    pub timestamps: bool,
    /// Whether to log to stdout.
    pub stdout: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            timestamps: true,
            stdout: true,
        }
    }
}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// ---------------------------------------------------------------------------
// Config errors
// ---------------------------------------------------------------------------

/// A configuration validation error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

// ---------------------------------------------------------------------------
// Config versioning
// ---------------------------------------------------------------------------

/// A versioned configuration snapshot for auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    pub version: u64,
    pub config: SdkConfig,
    pub created_at: DateTime<Utc>,
    pub description: String,
}

/// Manages configuration versions.
pub struct ConfigHistory {
    snapshots: Vec<ConfigSnapshot>,
    current_version: u64,
}

impl ConfigHistory {
    pub fn new(initial: SdkConfig) -> Self {
        let snapshot = ConfigSnapshot {
            version: 1,
            config: initial,
            created_at: Utc::now(),
            description: "initial configuration".into(),
        };
        Self {
            snapshots: vec![snapshot],
            current_version: 1,
        }
    }

    /// Save a new configuration version.
    pub fn save(&mut self, config: SdkConfig, description: impl Into<String>) -> u64 {
        self.current_version += 1;
        self.snapshots.push(ConfigSnapshot {
            version: self.current_version,
            config,
            created_at: Utc::now(),
            description: description.into(),
        });
        debug!(version = self.current_version, "config snapshot saved");
        self.current_version
    }

    /// Get the current version number.
    pub fn current_version(&self) -> u64 {
        self.current_version
    }

    /// Get a specific version's snapshot.
    pub fn get(&self, version: u64) -> Option<&ConfigSnapshot> {
        self.snapshots.iter().find(|s| s.version == version)
    }

    /// Get the latest configuration.
    pub fn latest(&self) -> &SdkConfig {
        &self.snapshots.last().unwrap().config
    }

    /// Get the number of snapshots.
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    /// Roll back to a previous version. Returns the config if found.
    pub fn rollback(&mut self, version: u64) -> Option<SdkConfig> {
        let config = self.get(version)?.config.clone();
        self.save(config.clone(), format!("rollback to v{}", version));
        Some(config)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_validates() {
        let config = SdkConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn empty_endpoint_fails_validation() {
        let config = SdkConfig {
            default_endpoint: String::new(),
            ..SdkConfig::default()
        };
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.field == "default_endpoint"));
    }

    #[test]
    fn zero_timeout_fails_validation() {
        let config = SdkConfig {
            request_timeout_ms: 0,
            ..SdkConfig::default()
        };
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.field == "request_timeout_ms"));
    }

    #[test]
    fn excessive_retries_fails_validation() {
        let config = SdkConfig {
            retry_policy: RetryPolicy {
                max_retries: 200,
                ..RetryPolicy::default()
            },
            ..SdkConfig::default()
        };
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.field == "retry_policy.max_retries"));
    }

    #[test]
    fn multiple_validation_errors() {
        let config = SdkConfig {
            default_endpoint: String::new(),
            request_timeout_ms: 0,
            max_plugins: 0,
            ..SdkConfig::default()
        };
        let errors = config.validate().unwrap_err();
        assert!(errors.len() >= 3);
    }

    #[test]
    fn development_config() {
        let config = SdkConfig::development();
        assert!(config.features.debug_mode);
        assert!(config.default_endpoint.contains("localhost"));
        assert_eq!(config.retry_policy.max_retries, 0);
    }

    #[test]
    fn production_config() {
        let config = SdkConfig::production();
        assert!(!config.features.debug_mode);
        assert!(config.default_endpoint.contains("aurora-nav"));
        assert!(config.retry_policy.max_retries > 0);
    }

    #[test]
    fn retry_policy_fixed_delay() {
        let policy = RetryPolicy {
            max_retries: 5,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
            strategy: BackoffStrategy::Fixed,
        };
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 100);
        assert_eq!(policy.delay_for_attempt(4), 100);
        assert_eq!(policy.delay_for_attempt(5), 0); // beyond max_retries
    }

    #[test]
    fn retry_policy_linear_delay() {
        let policy = RetryPolicy {
            max_retries: 5,
            base_delay_ms: 100,
            max_delay_ms: 10_000,
            strategy: BackoffStrategy::Linear,
        };
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 300);
    }

    #[test]
    fn retry_policy_exponential_delay() {
        let policy = RetryPolicy {
            max_retries: 10,
            base_delay_ms: 100,
            max_delay_ms: 5_000,
            strategy: BackoffStrategy::Exponential,
        };
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 400);
        assert_eq!(policy.delay_for_attempt(3), 800);
        // Capped at max_delay.
        assert_eq!(policy.delay_for_attempt(8), 5_000);
    }

    #[test]
    fn retry_policy_should_retry() {
        let policy = RetryPolicy {
            max_retries: 3,
            ..RetryPolicy::default()
        };
        assert!(policy.should_retry(0));
        assert!(policy.should_retry(2));
        assert!(!policy.should_retry(3));
        assert!(!policy.should_retry(10));
    }

    #[test]
    fn config_history_tracks_versions() {
        let initial = SdkConfig::default();
        let mut history = ConfigHistory::new(initial);
        assert_eq!(history.current_version(), 1);
        assert_eq!(history.count(), 1);

        let v2 = history.save(SdkConfig::development(), "switch to dev");
        assert_eq!(v2, 2);
        assert_eq!(history.count(), 2);

        let snapshot = history.get(1).unwrap();
        assert_eq!(snapshot.version, 1);
        assert_eq!(snapshot.description, "initial configuration");
    }

    #[test]
    fn config_history_rollback() {
        let mut history = ConfigHistory::new(SdkConfig::production());
        history.save(SdkConfig::development(), "switch to dev");
        assert_eq!(history.current_version(), 2);

        // Rollback to v1.
        let rolled_back = history.rollback(1).unwrap();
        assert_eq!(history.current_version(), 3);
        assert_eq!(history.count(), 3);
        // The rolled-back config should be production.
        assert!(rolled_back.default_endpoint.contains("aurora-nav"));
    }

    #[test]
    fn config_history_rollback_nonexistent() {
        let mut history = ConfigHistory::new(SdkConfig::default());
        assert!(history.rollback(99).is_none());
    }

    #[test]
    fn config_history_latest() {
        let mut history = ConfigHistory::new(SdkConfig::default());
        history.save(SdkConfig::development(), "dev");
        let latest = history.latest();
        assert!(latest.default_endpoint.contains("localhost"));
    }
}
