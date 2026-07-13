//! Fluent builder for constructing `AuroraConfig` programmatically.

use crate::sections::AuroraConfig;
use crate::validate::{self, ConfigError, ValidationResult};

/// Builder for constructing an `AuroraConfig` with a fluent API.
pub struct ConfigBuilder {
    config: AuroraConfig,
}

impl ConfigBuilder {
    /// Start with default configuration.
    pub fn new() -> Self {
        Self {
            config: AuroraConfig::default(),
        }
    }

    /// Set the instance name.
    pub fn instance_name(mut self, name: &str) -> Self {
        self.config.system.instance_name = name.into();
        self
    }

    /// Set the log level.
    pub fn log_level(mut self, level: &str) -> Self {
        self.config.system.log_level = level.into();
        self
    }

    /// Set the number of worker threads.
    pub fn worker_threads(mut self, n: usize) -> Self {
        self.config.system.worker_threads = n;
        self
    }

    /// Set the API port.
    pub fn api_port(mut self, port: u16) -> Self {
        self.config.api.port = port;
        self
    }

    /// Set the API host.
    pub fn api_host(mut self, host: &str) -> Self {
        self.config.api.host = host.into();
        self
    }

    /// Set minimum GNSS satellites.
    pub fn min_satellites(mut self, n: usize) -> Self {
        self.config.gnss.min_satellites = n;
        self
    }

    /// Set the GNSS elevation mask.
    pub fn elevation_mask_deg(mut self, deg: f64) -> Self {
        self.config.gnss.elevation_mask_deg = deg;
        self
    }

    /// Enable or disable GPS.
    pub fn enable_gps(mut self, v: bool) -> Self {
        self.config.gnss.enable_gps = v;
        self
    }

    /// Enable or disable Galileo.
    pub fn enable_galileo(mut self, v: bool) -> Self {
        self.config.gnss.enable_galileo = v;
        self
    }

    /// Enable or disable GLONASS.
    pub fn enable_glonass(mut self, v: bool) -> Self {
        self.config.gnss.enable_glonass = v;
        self
    }

    /// Enable or disable BeiDou.
    pub fn enable_beidou(mut self, v: bool) -> Self {
        self.config.gnss.enable_beidou = v;
        self
    }

    /// Set telemetry buffer capacity.
    pub fn telemetry_buffer(mut self, capacity: usize) -> Self {
        self.config.telemetry.buffer_capacity = capacity;
        self
    }

    /// Enable or disable fleet management.
    pub fn fleet_enabled(mut self, v: bool) -> Self {
        self.config.fleet.enabled = v;
        self
    }

    /// Enable or disable satellite fallback.
    pub fn satellite_enabled(mut self, v: bool) -> Self {
        self.config.satellite.enabled = v;
        self
    }

    /// Enable or disable digital twin.
    pub fn twin_enabled(mut self, v: bool) -> Self {
        self.config.twin.enabled = v;
        self
    }

    /// Enable or disable vehicle integrations.
    pub fn vehicle_enabled(mut self, v: bool) -> Self {
        self.config.vehicle.enabled = v;
        self
    }

    /// Enable or disable payments.
    pub fn payments_enabled(mut self, v: bool) -> Self {
        self.config.payments.enabled = v;
        self
    }

    /// Set fusion process noise scale.
    pub fn process_noise_scale(mut self, scale: f64) -> Self {
        self.config.fusion.process_noise_scale = scale;
        self
    }

    /// Set routing risk aversion.
    pub fn risk_aversion(mut self, v: f64) -> Self {
        self.config.routing.risk_aversion = v;
        self
    }

    /// Set routing max alternatives.
    pub fn max_alternatives(mut self, n: usize) -> Self {
        self.config.routing.max_alternatives = n;
        self
    }

    /// Build and validate the configuration.
    pub fn build(self) -> Result<(AuroraConfig, ValidationResult), ConfigError> {
        let result = validate::validate(&self.config)?;
        Ok((self.config, result))
    }

    /// Build without validation (for testing).
    pub fn build_unchecked(self) -> AuroraConfig {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults_pass_validation() {
        let (cfg, result) = ConfigBuilder::new().build().unwrap();
        assert_eq!(cfg.api.port, 3000);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn builder_overrides_apply() {
        let (cfg, _) = ConfigBuilder::new()
            .api_port(9090)
            .instance_name("test-node")
            .min_satellites(6)
            .log_level("debug")
            .worker_threads(8)
            .telemetry_buffer(10_000)
            .build()
            .unwrap();
        assert_eq!(cfg.api.port, 9090);
        assert_eq!(cfg.system.instance_name, "test-node");
        assert_eq!(cfg.gnss.min_satellites, 6);
        assert_eq!(cfg.system.log_level, "debug");
        assert_eq!(cfg.system.worker_threads, 8);
        assert_eq!(cfg.telemetry.buffer_capacity, 10_000);
    }

    #[test]
    fn builder_validation_rejects_invalid() {
        let result = ConfigBuilder::new().min_satellites(1).build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_unchecked_skips_validation() {
        let cfg = ConfigBuilder::new().min_satellites(1).build_unchecked();
        assert_eq!(cfg.gnss.min_satellites, 1);
    }

    #[test]
    fn builder_constellation_toggles() {
        let (cfg, _) = ConfigBuilder::new()
            .enable_gps(false)
            .enable_galileo(false)
            .enable_glonass(true)
            .enable_beidou(true)
            .build()
            .unwrap();
        assert!(!cfg.gnss.enable_gps);
        assert!(!cfg.gnss.enable_galileo);
        assert!(cfg.gnss.enable_glonass);
        assert!(cfg.gnss.enable_beidou);
    }

    #[test]
    fn builder_feature_toggles() {
        let (cfg, _) = ConfigBuilder::new()
            .fleet_enabled(true)
            .satellite_enabled(true)
            .twin_enabled(true)
            .vehicle_enabled(true)
            .payments_enabled(true)
            .build()
            .unwrap();
        assert!(cfg.fleet.enabled);
        assert!(cfg.satellite.enabled);
        assert!(cfg.twin.enabled);
        assert!(cfg.vehicle.enabled);
        assert!(cfg.payments.enabled);
    }
}
