//! Configuration loading from files and environment variables.

use crate::sections::AuroraConfig;
use crate::validate::{self, ConfigError, ValidationResult};
use std::path::Path;

/// Load configuration from a TOML file, apply environment overrides,
/// and validate.
pub fn load_config(path: &Path) -> Result<(AuroraConfig, ValidationResult), ConfigError> {
    let content = std::fs::read_to_string(path)?;
    let mut config: AuroraConfig = toml::from_str(&content)?;
    apply_env_overrides(&mut config)?;
    let result = validate::validate(&config)?;
    Ok((config, result))
}

/// Load configuration from a TOML string (useful for tests and embedded
/// configs).
pub fn load_from_str(toml_str: &str) -> Result<(AuroraConfig, ValidationResult), ConfigError> {
    let mut config: AuroraConfig = toml::from_str(toml_str)?;
    apply_env_overrides(&mut config)?;
    let result = validate::validate(&config)?;
    Ok((config, result))
}

/// Load the default configuration (no file needed), applying any
/// `AURORA_*` environment variable overrides.
pub fn load_default() -> Result<(AuroraConfig, ValidationResult), ConfigError> {
    let mut config = AuroraConfig::default();
    apply_env_overrides(&mut config)?;
    let result = validate::validate(&config)?;
    Ok((config, result))
}

/// Apply `AURORA_*` environment variable overrides.
///
/// Supported overrides:
/// - `AURORA_API_PORT` → `api.port`
/// - `AURORA_API_HOST` → `api.host`
/// - `AURORA_LOG_LEVEL` → `system.log_level`
/// - `AURORA_WORKER_THREADS` → `system.worker_threads`
/// - `AURORA_INSTANCE_NAME` → `system.instance_name`
/// - `AURORA_GNSS_MIN_SATELLITES` → `gnss.min_satellites`
/// - `AURORA_TELEMETRY_BUFFER` → `telemetry.buffer_capacity`
pub fn apply_env_overrides(config: &mut AuroraConfig) -> Result<(), ConfigError> {
    if let Ok(val) = std::env::var("AURORA_API_PORT") {
        config.api.port = val
            .parse()
            .map_err(|_| ConfigError::EnvOverride("AURORA_API_PORT must be a u16".into()))?;
    }
    if let Ok(val) = std::env::var("AURORA_API_HOST") {
        config.api.host = val;
    }
    if let Ok(val) = std::env::var("AURORA_LOG_LEVEL") {
        config.system.log_level = val;
    }
    if let Ok(val) = std::env::var("AURORA_WORKER_THREADS") {
        config.system.worker_threads = val.parse().map_err(|_| {
            ConfigError::EnvOverride("AURORA_WORKER_THREADS must be a usize".into())
        })?;
    }
    if let Ok(val) = std::env::var("AURORA_INSTANCE_NAME") {
        config.system.instance_name = val;
    }
    if let Ok(val) = std::env::var("AURORA_GNSS_MIN_SATELLITES") {
        config.gnss.min_satellites = val.parse().map_err(|_| {
            ConfigError::EnvOverride("AURORA_GNSS_MIN_SATELLITES must be a usize".into())
        })?;
    }
    if let Ok(val) = std::env::var("AURORA_TELEMETRY_BUFFER") {
        config.telemetry.buffer_capacity = val.parse().map_err(|_| {
            ConfigError::EnvOverride("AURORA_TELEMETRY_BUFFER must be a usize".into())
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_str_with_defaults() {
        let (cfg, result) = load_from_str("").unwrap();
        assert_eq!(cfg.api.port, 3000);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn load_from_str_with_overrides() {
        let toml_str = r#"
[api]
port = 9090

[gnss]
min_satellites = 5
"#;
        let (cfg, _) = load_from_str(toml_str).unwrap();
        assert_eq!(cfg.api.port, 9090);
        assert_eq!(cfg.gnss.min_satellites, 5);
    }

    #[test]
    fn load_default_passes() {
        let (cfg, result) = load_default().unwrap();
        assert_eq!(cfg.api.port, 3000);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn invalid_toml_returns_error() {
        let result = load_from_str("invalid = [[[");
        assert!(result.is_err());
    }

    #[test]
    fn validation_error_propagates() {
        let toml_str = r#"
[gnss]
min_satellites = 1
"#;
        let result = load_from_str(toml_str);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("min_satellites"));
    }
}
