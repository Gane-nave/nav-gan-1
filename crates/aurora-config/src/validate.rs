//! Configuration validation.

use crate::sections::AuroraConfig;

/// Errors that can occur during configuration loading or validation.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("environment override error: {0}")]
    EnvOverride(String),
}

/// Validation result containing zero or more warnings.
#[derive(Debug)]
pub struct ValidationResult {
    pub warnings: Vec<String>,
}

/// Validate a configuration, returning warnings for questionable values
/// and an error for invalid values.
pub fn validate(config: &AuroraConfig) -> Result<ValidationResult, ConfigError> {
    let mut warnings = Vec::new();

    // GNSS validation
    if config.gnss.min_satellites < 4 {
        return Err(ConfigError::Validation(
            "gnss.min_satellites must be >= 4 for a valid 3-D fix".into(),
        ));
    }
    if config.gnss.elevation_mask_deg < 0.0 || config.gnss.elevation_mask_deg > 90.0 {
        return Err(ConfigError::Validation(
            "gnss.elevation_mask_deg must be in [0, 90]".into(),
        ));
    }

    // Fusion validation
    if config.fusion.process_noise_scale <= 0.0 {
        return Err(ConfigError::Validation(
            "fusion.process_noise_scale must be > 0".into(),
        ));
    }
    if config.fusion.min_confidence < 0.0 || config.fusion.min_confidence > 1.0 {
        return Err(ConfigError::Validation(
            "fusion.min_confidence must be in [0, 1]".into(),
        ));
    }

    // Integrity validation
    if config.integrity.min_trust_score < 0.0 || config.integrity.min_trust_score > 1.0 {
        return Err(ConfigError::Validation(
            "integrity.min_trust_score must be in [0, 1]".into(),
        ));
    }

    // Routing validation
    if config.routing.max_alternatives == 0 {
        return Err(ConfigError::Validation(
            "routing.max_alternatives must be >= 1".into(),
        ));
    }
    if config.routing.risk_aversion < 0.0 || config.routing.risk_aversion > 1.0 {
        warnings.push("routing.risk_aversion outside [0,1] — clamped values may be used".into());
    }

    // API validation
    if config.api.port == 0 {
        return Err(ConfigError::Validation("api.port must be > 0".into()));
    }

    // System validation
    if config.system.worker_threads == 0 {
        return Err(ConfigError::Validation(
            "system.worker_threads must be >= 1".into(),
        ));
    }

    // Telemetry validation
    if config.telemetry.buffer_capacity == 0 {
        warnings.push("telemetry.buffer_capacity is 0 — telemetry will be disabled".into());
    }

    // Twin validation
    if config.twin.enabled && config.twin.time_step_s <= 0.0 {
        return Err(ConfigError::Validation(
            "twin.time_step_s must be > 0 when twin is enabled".into(),
        ));
    }

    // Edge validation
    if config.edge.enabled && config.edge.model_memory_bytes == 0 {
        return Err(ConfigError::Validation(
            "edge.model_memory_bytes must be > 0 when edge is enabled".into(),
        ));
    }

    Ok(ValidationResult { warnings })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_passes_validation() {
        let cfg = AuroraConfig::default();
        let result = validate(&cfg).unwrap();
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn min_satellites_below_4_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.gnss.min_satellites = 2;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("min_satellites"));
    }

    #[test]
    fn zero_port_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.api.port = 0;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("port"));
    }

    #[test]
    fn zero_worker_threads_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.system.worker_threads = 0;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("worker_threads"));
    }

    #[test]
    fn negative_process_noise_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.fusion.process_noise_scale = -1.0;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("process_noise_scale"));
    }

    #[test]
    fn zero_buffer_capacity_warns() {
        let mut cfg = AuroraConfig::default();
        cfg.telemetry.buffer_capacity = 0;
        let result = validate(&cfg).unwrap();
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("buffer_capacity"));
    }

    #[test]
    fn enabled_twin_with_zero_timestep_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.twin.enabled = true;
        cfg.twin.time_step_s = 0.0;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("time_step_s"));
    }

    #[test]
    fn elevation_mask_out_of_range_fails() {
        let mut cfg = AuroraConfig::default();
        cfg.gnss.elevation_mask_deg = 100.0;
        let err = validate(&cfg).unwrap_err();
        assert!(err.to_string().contains("elevation_mask_deg"));
    }
}
