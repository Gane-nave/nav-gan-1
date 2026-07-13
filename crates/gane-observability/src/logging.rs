//! Structured logging configuration using tracing-subscriber.

use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

/// Log output format.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LogFormat {
    /// Human-readable, colored output (default for development).
    #[default]
    Pretty,
    /// Compact single-line output.
    Compact,
    /// JSON-structured output (recommended for production).
    Json,
}

/// Logging configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogConfig {
    /// Minimum log level (trace, debug, info, warn, error).
    pub level: String,
    /// Output format.
    pub format: LogFormat,
    /// Whether to include source file/line in log output.
    pub with_file: bool,
    /// Whether to include the target module path.
    pub with_target: bool,
    /// Whether to include thread names.
    pub with_thread_names: bool,
    /// Whether to include thread IDs.
    pub with_thread_ids: bool,
    /// Whether to include span events (new/close).
    pub with_span_events: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: LogFormat::Pretty,
            with_file: false,
            with_target: true,
            with_thread_names: false,
            with_thread_ids: false,
            with_span_events: false,
        }
    }
}

/// Valid log level strings.
const VALID_LEVELS: &[&str] = &["trace", "debug", "info", "warn", "error", "off"];

/// Validate that the log level string is a known level or a valid EnvFilter directive.
fn validate_level(level: &str) -> Result<(), String> {
    let lower = level.to_lowercase();
    if VALID_LEVELS.contains(&lower.as_str()) {
        return Ok(());
    }
    // Allow valid EnvFilter directives (e.g. "gane_api=debug,info")
    if lower.contains('=') || lower.contains(',') {
        return Ok(());
    }
    Err(format!(
        "Invalid log level '{}': must be one of {:?}",
        level, VALID_LEVELS
    ))
}

/// Initialise the global tracing subscriber.
///
/// Call this once at application startup. Subsequent calls are no-ops
/// (tracing-subscriber's `try_init` returns `Err` if already set).
pub fn init_logging(config: &LogConfig) -> Result<(), String> {
    validate_level(&config.level)?;

    let env_filter = EnvFilter::try_new(&config.level)
        .map_err(|e| format!("Invalid log level '{}': {}", config.level, e))?;

    match config.format {
        LogFormat::Json => {
            let builder = fmt()
                .json()
                .with_env_filter(env_filter)
                .with_file(config.with_file)
                .with_target(config.with_target)
                .with_thread_names(config.with_thread_names)
                .with_thread_ids(config.with_thread_ids);
            builder
                .try_init()
                .map_err(|e| format!("Logger init failed: {}", e))
        }
        LogFormat::Compact => {
            let builder = fmt()
                .compact()
                .with_env_filter(env_filter)
                .with_file(config.with_file)
                .with_target(config.with_target)
                .with_thread_names(config.with_thread_names)
                .with_thread_ids(config.with_thread_ids);
            builder
                .try_init()
                .map_err(|e| format!("Logger init failed: {}", e))
        }
        LogFormat::Pretty => {
            let builder = fmt()
                .with_env_filter(env_filter)
                .with_file(config.with_file)
                .with_target(config.with_target)
                .with_thread_names(config.with_thread_names)
                .with_thread_ids(config.with_thread_ids);
            builder
                .try_init()
                .map_err(|e| format!("Logger init failed: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = LogConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, LogFormat::Pretty);
        assert!(!config.with_file);
        assert!(config.with_target);
    }

    #[test]
    fn log_format_default() {
        assert_eq!(LogFormat::default(), LogFormat::Pretty);
    }

    #[test]
    fn invalid_level_returns_error() {
        let config = LogConfig {
            level: "not_a_level".to_string(),
            ..Default::default()
        };
        let result = init_logging(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid log level"));
    }

    #[test]
    fn validate_known_levels() {
        for level in &["trace", "debug", "info", "warn", "error", "off"] {
            assert!(validate_level(level).is_ok());
        }
    }

    #[test]
    fn validate_rejects_garbage() {
        assert!(validate_level("banana").is_err());
        assert!(validate_level("").is_err());
    }

    #[test]
    fn json_format_serialization() {
        let config = LogConfig {
            format: LogFormat::Json,
            ..Default::default()
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"Json\""));
    }

    #[test]
    fn config_roundtrip() {
        let config = LogConfig {
            level: "debug".to_string(),
            format: LogFormat::Compact,
            with_file: true,
            with_target: false,
            with_thread_names: true,
            with_thread_ids: true,
            with_span_events: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: LogConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.level, "debug");
        assert_eq!(restored.format, LogFormat::Compact);
        assert!(restored.with_file);
        assert!(!restored.with_target);
        assert!(restored.with_thread_names);
        assert!(restored.with_thread_ids);
        assert!(restored.with_span_events);
    }
}
