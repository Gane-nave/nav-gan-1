//! Strongly-typed configuration sections for every G.A.N.E NAV subsystem.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Root configuration
// ---------------------------------------------------------------------------

/// Top-level configuration for the entire G.A.N.E NAV system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuroraConfig {
    /// General system settings.
    #[serde(default)]
    pub system: SystemConfig,

    /// GNSS acquisition settings.
    #[serde(default)]
    pub gnss: GnssConfig,

    /// Correction layer settings.
    #[serde(default)]
    pub corrections: CorrectionConfig,

    /// Sensor fusion settings.
    #[serde(default)]
    pub fusion: FusionConfig,

    /// Integrity monitoring settings.
    #[serde(default)]
    pub integrity: IntegrityConfig,

    /// Continuity / fallback settings.
    #[serde(default)]
    pub continuity: ContinuityConfig,

    /// Routing engine settings.
    #[serde(default)]
    pub routing: RoutingConfig,

    /// Map engine settings.
    #[serde(default)]
    pub map: MapConfig,

    /// Traffic & stability settings.
    #[serde(default)]
    pub traffic: TrafficConfig,

    /// UX / display settings.
    #[serde(default)]
    pub ux: UxConfig,

    /// Fleet management settings.
    #[serde(default)]
    pub fleet: FleetConfig,

    /// Emergency system settings.
    #[serde(default)]
    pub emergency: EmergencyConfig,

    /// Offline resilience settings.
    #[serde(default)]
    pub offline: OfflineConfig,

    /// Edge processing settings.
    #[serde(default)]
    pub edge: EdgeConfig,

    /// Satellite communication settings.
    #[serde(default)]
    pub satellite: SatelliteConfig,

    /// Smart city integration settings.
    #[serde(default)]
    pub city: CityConfig,

    /// Digital twin settings.
    #[serde(default)]
    pub twin: TwinConfig,

    /// Developer platform settings.
    #[serde(default)]
    pub developer: DeveloperConfig,

    /// Marketplace settings.
    #[serde(default)]
    pub marketplace: MarketplaceConfig,

    /// Payment system settings.
    #[serde(default)]
    pub payments: PaymentConfig,

    /// Vehicle integration settings.
    #[serde(default)]
    pub vehicle: VehicleConfig,

    /// API server settings.
    #[serde(default)]
    pub api: ApiConfig,

    /// Telemetry / observability settings.
    #[serde(default)]
    pub telemetry: TelemetryConfig,
}

// ---------------------------------------------------------------------------
// Section: System
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SystemConfig {
    /// Human-readable instance name.
    pub instance_name: String,
    /// Log level (trace, debug, info, warn, error).
    pub log_level: String,
    /// Enable JSON-structured logging.
    pub json_logging: bool,
    /// Maximum worker threads for the async runtime.
    pub worker_threads: usize,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            instance_name: "gane-nav-default".into(),
            log_level: "info".into(),
            json_logging: false,
            worker_threads: 4,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: GNSS
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GnssConfig {
    /// Enable GPS constellation.
    pub enable_gps: bool,
    /// Enable Galileo constellation.
    pub enable_galileo: bool,
    /// Enable GLONASS constellation.
    pub enable_glonass: bool,
    /// Enable BeiDou constellation.
    pub enable_beidou: bool,
    /// Minimum satellites for a valid fix.
    pub min_satellites: usize,
    /// Maximum age of a measurement before discard (seconds).
    pub max_measurement_age_s: f64,
    /// Elevation mask (degrees) — satellites below this are excluded.
    pub elevation_mask_deg: f64,
}

impl Default for GnssConfig {
    fn default() -> Self {
        Self {
            enable_gps: true,
            enable_galileo: true,
            enable_glonass: true,
            enable_beidou: true,
            min_satellites: 4,
            max_measurement_age_s: 10.0,
            elevation_mask_deg: 10.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Corrections
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CorrectionConfig {
    /// Enable SBAS corrections.
    pub enable_sbas: bool,
    /// Enable PPP corrections.
    pub enable_ppp: bool,
    /// Enable RTK corrections.
    pub enable_rtk: bool,
    /// Enable NRTK corrections.
    pub enable_nrtk: bool,
    /// Maximum correction age before stale (seconds).
    pub max_correction_age_s: f64,
}

impl Default for CorrectionConfig {
    fn default() -> Self {
        Self {
            enable_sbas: true,
            enable_ppp: true,
            enable_rtk: true,
            enable_nrtk: true,
            max_correction_age_s: 30.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Fusion
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FusionConfig {
    /// Process noise covariance scale factor.
    pub process_noise_scale: f64,
    /// Measurement noise covariance scale factor.
    pub measurement_noise_scale: f64,
    /// Minimum confidence to accept a fused solution.
    pub min_confidence: f64,
    /// Enable IMU integration.
    pub enable_imu: bool,
    /// Enable odometry integration.
    pub enable_odometry: bool,
    /// Enable map matching constraints.
    pub enable_map_matching: bool,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            process_noise_scale: 1.0,
            measurement_noise_scale: 1.0,
            min_confidence: 0.3,
            enable_imu: true,
            enable_odometry: true,
            enable_map_matching: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Integrity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IntegrityConfig {
    /// Enable jamming detection.
    pub enable_jamming_detection: bool,
    /// Enable spoofing detection.
    pub enable_spoofing_detection: bool,
    /// Enable multipath detection.
    pub enable_multipath_detection: bool,
    /// Minimum trust score to consider a source reliable.
    pub min_trust_score: f64,
    /// Trust decay rate per second.
    pub trust_decay_rate: f64,
}

impl Default for IntegrityConfig {
    fn default() -> Self {
        Self {
            enable_jamming_detection: true,
            enable_spoofing_detection: true,
            enable_multipath_detection: true,
            min_trust_score: 0.5,
            trust_decay_rate: 0.01,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Continuity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContinuityConfig {
    /// Timeout before escalating continuity mode (seconds).
    pub escalation_timeout_s: f64,
    /// Maximum time in degraded mode before emergency (seconds).
    pub max_degraded_time_s: f64,
    /// Enable automatic recovery when sources return.
    pub auto_recovery: bool,
}

impl Default for ContinuityConfig {
    fn default() -> Self {
        Self {
            escalation_timeout_s: 5.0,
            max_degraded_time_s: 30.0,
            auto_recovery: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Routing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RoutingConfig {
    /// Maximum alternative routes to generate.
    pub max_alternatives: usize,
    /// Enable probabilistic routing.
    pub enable_probabilistic: bool,
    /// Risk aversion factor (0.0 = risk-neutral, 1.0 = risk-averse).
    pub risk_aversion: f64,
    /// Enable real-time traffic rerouting.
    pub enable_live_rerouting: bool,
    /// Rerouting check interval (seconds).
    pub reroute_interval_s: f64,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            max_alternatives: 3,
            enable_probabilistic: true,
            risk_aversion: 0.5,
            enable_live_rerouting: true,
            reroute_interval_s: 30.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Map
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MapConfig {
    /// Maximum tile cache size in bytes.
    pub tile_cache_bytes: u64,
    /// Tile TTL before refresh (seconds).
    pub tile_ttl_s: u64,
    /// Enable map matching.
    pub enable_map_matching: bool,
    /// Map matching search radius (metres).
    pub matching_radius_m: f64,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            tile_cache_bytes: 256 * 1024 * 1024, // 256 MB
            tile_ttl_s: 86400,                   // 24 hours
            enable_map_matching: true,
            matching_radius_m: 30.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Traffic
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TrafficConfig {
    /// Enable traffic flow controller.
    pub enable_flow_control: bool,
    /// Enable herd suppression.
    pub enable_herd_suppression: bool,
    /// Traffic forecast horizon (seconds).
    pub forecast_horizon_s: u64,
    /// Network stability oscillation damping factor.
    pub oscillation_damping: f64,
}

impl Default for TrafficConfig {
    fn default() -> Self {
        Self {
            enable_flow_control: true,
            enable_herd_suppression: true,
            forecast_horizon_s: 3600,
            oscillation_damping: 0.7,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: UX
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UxConfig {
    /// Default driving mode.
    pub default_mode: String,
    /// Enable cognitive-load auto-simplification.
    pub auto_simplify: bool,
    /// Default theme.
    pub default_theme: String,
    /// Enable micro-navigation (gate-level arrival).
    pub enable_micro_nav: bool,
}

impl Default for UxConfig {
    fn default() -> Self {
        Self {
            default_mode: "City".into(),
            auto_simplify: true,
            default_theme: "Day".into(),
            enable_micro_nav: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Fleet
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FleetConfig {
    /// Enable fleet management features.
    pub enabled: bool,
    /// Maximum vehicles per fleet.
    pub max_vehicles: usize,
    /// SLA check interval (seconds).
    pub sla_check_interval_s: u64,
    /// Default assignment strategy.
    pub assignment_strategy: String,
}

impl Default for FleetConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_vehicles: 1000,
            sla_check_interval_s: 60,
            assignment_strategy: "NearestFirst".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Emergency
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EmergencyConfig {
    /// Enable emergency corridor routing.
    pub enable_corridor: bool,
    /// Enable signal preemption requests.
    pub enable_preemption: bool,
    /// Maximum concurrent incidents.
    pub max_incidents: usize,
}

impl Default for EmergencyConfig {
    fn default() -> Self {
        Self {
            enable_corridor: true,
            enable_preemption: true,
            max_incidents: 50,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Offline
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OfflineConfig {
    /// Enable offline-first mode.
    pub enabled: bool,
    /// Offline storage budget (bytes).
    pub storage_budget_bytes: u64,
    /// Queue TTL for offline messages (seconds).
    pub queue_ttl_s: u64,
    /// Enable deterministic sync on reconnect.
    pub deterministic_sync: bool,
}

impl Default for OfflineConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_budget_bytes: 512 * 1024 * 1024, // 512 MB
            queue_ttl_s: 86400,
            deterministic_sync: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Edge
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EdgeConfig {
    /// Enable edge processing pipeline.
    pub enabled: bool,
    /// Maximum memory for ML models (bytes).
    pub model_memory_bytes: u64,
    /// Enable data reduction (sampling / delta / spatial).
    pub enable_reduction: bool,
    /// Device health check interval (seconds).
    pub health_check_interval_s: u64,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            model_memory_bytes: 128 * 1024 * 1024, // 128 MB
            enable_reduction: true,
            health_check_interval_s: 30,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Satellite
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SatelliteConfig {
    /// Enable satellite communication fallback.
    pub enabled: bool,
    /// Preferred link type (LEO or GEO).
    pub preferred_link: String,
    /// Maximum packet size (bytes).
    pub max_packet_bytes: usize,
    /// Store-and-forward buffer size.
    pub buffer_capacity: usize,
}

impl Default for SatelliteConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            preferred_link: "LEO".into(),
            max_packet_bytes: 340,
            buffer_capacity: 1000,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: City
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CityConfig {
    /// Enable smart city signal integration.
    pub enable_signals: bool,
    /// Enable infrastructure health monitoring.
    pub enable_infrastructure: bool,
    /// Enable zone management.
    pub enable_zones: bool,
    /// Green wave speed range (km/h).
    pub green_wave_speed_range: (f64, f64),
}

impl Default for CityConfig {
    fn default() -> Self {
        Self {
            enable_signals: true,
            enable_infrastructure: true,
            enable_zones: true,
            green_wave_speed_range: (30.0, 60.0),
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Twin
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TwinConfig {
    /// Enable digital twin simulation.
    pub enabled: bool,
    /// Maximum entities in the twin registry.
    pub max_entities: usize,
    /// Simulation time step (seconds).
    pub time_step_s: f64,
    /// Enable scenario sweep mode.
    pub enable_scenarios: bool,
}

impl Default for TwinConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_entities: 10_000,
            time_step_s: 1.0,
            enable_scenarios: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Developer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeveloperConfig {
    /// Enable developer platform.
    pub enabled: bool,
    /// Maximum API keys per developer.
    pub max_api_keys: usize,
    /// Rate limit (requests per second).
    pub rate_limit_rps: u32,
    /// Webhook retry count.
    pub webhook_max_retries: u32,
}

impl Default for DeveloperConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_api_keys: 10,
            rate_limit_rps: 100,
            webhook_max_retries: 3,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Marketplace
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MarketplaceConfig {
    /// Enable marketplace.
    pub enabled: bool,
    /// Maximum listings per author.
    pub max_listings_per_author: usize,
    /// Review moderation enabled.
    pub enable_review_moderation: bool,
    /// Minimum version stability for auto-install.
    pub min_auto_install_stability: String,
}

impl Default for MarketplaceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_listings_per_author: 50,
            enable_review_moderation: true,
            min_auto_install_stability: "Stable".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Payments
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PaymentConfig {
    /// Enable payment processing.
    pub enabled: bool,
    /// Default currency.
    pub default_currency: String,
    /// Invoice due days.
    pub invoice_due_days: u32,
    /// Enable automatic billing.
    pub auto_billing: bool,
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default_currency: "USD".into(),
            invoice_due_days: 30,
            auto_billing: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Vehicle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VehicleConfig {
    /// Enable vehicle manufacturer integrations.
    pub enabled: bool,
    /// Enable OBD-II diagnostics.
    pub enable_obd: bool,
    /// Enable CAN bus integration.
    pub enable_can: bool,
    /// Sensor polling interval (milliseconds).
    pub sensor_poll_ms: u64,
}

impl Default for VehicleConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            enable_obd: true,
            enable_can: true,
            sensor_poll_ms: 100,
        }
    }
}

// ---------------------------------------------------------------------------
// Section: API
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    /// Bind host.
    pub host: String,
    /// Bind port.
    pub port: u16,
    /// Enable CORS.
    pub enable_cors: bool,
    /// Enable request tracing.
    pub enable_tracing: bool,
    /// Request body size limit (bytes).
    pub max_body_bytes: usize,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 3000,
            enable_cors: true,
            enable_tracing: true,
            max_body_bytes: 10 * 1024 * 1024, // 10 MB
        }
    }
}

// ---------------------------------------------------------------------------
// Section: Telemetry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TelemetryConfig {
    /// Ring buffer capacity.
    pub buffer_capacity: usize,
    /// Enable audit logging.
    pub enable_audit: bool,
    /// Enable replay recording.
    pub enable_replay: bool,
    /// Flush interval (seconds).
    pub flush_interval_s: u64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            buffer_capacity: 50_000,
            enable_audit: true,
            enable_replay: false,
            flush_interval_s: 10,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = AuroraConfig::default();
        assert_eq!(cfg.api.port, 3000);
        assert!(cfg.gnss.enable_gps);
        assert!(cfg.gnss.enable_galileo);
        assert!(cfg.gnss.enable_glonass);
        assert!(cfg.gnss.enable_beidou);
        assert_eq!(cfg.gnss.min_satellites, 4);
        assert_eq!(cfg.system.worker_threads, 4);
        assert_eq!(cfg.telemetry.buffer_capacity, 50_000);
    }

    #[test]
    fn default_config_serialises_to_toml() {
        let cfg = AuroraConfig::default();
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        assert!(toml_str.contains("[gnss]"));
        assert!(toml_str.contains("enable_gps = true"));
        assert!(toml_str.contains("[api]"));
        assert!(toml_str.contains("port = 3000"));
    }

    #[test]
    fn default_config_round_trips_through_toml() {
        let original = AuroraConfig::default();
        let toml_str = toml::to_string_pretty(&original).unwrap();
        let parsed: AuroraConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.api.port, original.api.port);
        assert_eq!(parsed.gnss.min_satellites, original.gnss.min_satellites);
        assert_eq!(
            parsed.telemetry.buffer_capacity,
            original.telemetry.buffer_capacity
        );
    }

    #[test]
    fn partial_toml_fills_defaults() {
        let toml_str = r#"
[api]
port = 8080

[gnss]
min_satellites = 6
"#;
        let cfg: AuroraConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.api.port, 8080);
        assert_eq!(cfg.gnss.min_satellites, 6);
        // Other fields should be default
        assert!(cfg.gnss.enable_gps);
        assert_eq!(cfg.system.worker_threads, 4);
    }

    #[test]
    fn all_sections_present_in_default() {
        let cfg = AuroraConfig::default();
        // Just verify all sections are accessible with sensible defaults
        assert!(!cfg.system.instance_name.is_empty());
        assert!(cfg.corrections.enable_sbas);
        assert!(cfg.fusion.enable_imu);
        assert!(cfg.integrity.enable_jamming_detection);
        assert!(cfg.continuity.auto_recovery);
        assert!(cfg.routing.enable_probabilistic);
        assert!(cfg.map.enable_map_matching);
        assert!(cfg.traffic.enable_flow_control);
        assert!(cfg.ux.auto_simplify);
        assert!(!cfg.fleet.enabled); // fleet off by default
        assert!(cfg.emergency.enable_corridor);
        assert!(cfg.offline.enabled);
        assert!(cfg.edge.enabled);
        assert!(!cfg.satellite.enabled); // satellite off by default
        assert!(cfg.city.enable_signals);
        assert!(!cfg.twin.enabled); // twin off by default
        assert!(cfg.developer.enabled);
        assert!(cfg.marketplace.enabled);
        assert!(!cfg.payments.enabled); // payments off by default
        assert!(!cfg.vehicle.enabled); // vehicle off by default
        assert!(cfg.telemetry.enable_audit);
    }
}
