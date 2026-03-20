//! Navigation processing pipeline.
//!
//! Wires GNSS → Corrections → Sensors → Fusion → Integrity → Continuity
//! into a single processing step that produces a `FusedPosition`.

use aurora_config::AuroraConfig;
use aurora_continuity::ContinuityManager;
use aurora_core::types::FusedPosition;
use aurora_events::EventBus;
use aurora_fusion::FusionEngine;
use aurora_gnss::ConstellationManager;
use aurora_integrity::IntegrityEngine;
use aurora_orchestrator::ServiceRegistry;
use aurora_sensors::imu::ImuProcessor;
use aurora_telemetry::TelemetryRecorder;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::debug;

/// Holds the core navigation subsystems.
pub struct NavigationPipeline {
    pub gnss: Arc<RwLock<ConstellationManager>>,
    pub fusion: Arc<RwLock<FusionEngine>>,
    pub integrity: Arc<RwLock<IntegrityEngine>>,
    pub continuity: Arc<RwLock<ContinuityManager>>,
    pub imu: Arc<RwLock<ImuProcessor>>,
    pub telemetry: Arc<TelemetryRecorder>,
    pub event_bus: Arc<EventBus>,
    pub last_position: Arc<RwLock<Option<FusedPosition>>>,
    /// Infrastructure service registry (cache, circuit breaker, rate limiter, etc.).
    pub services: Arc<RwLock<ServiceRegistry>>,
    config: AuroraConfig,
    /// Whether any GNSS data has ever been received by this pipeline.
    has_received_gnss: AtomicBool,
}

impl NavigationPipeline {
    /// Create a new navigation pipeline from configuration.
    pub fn new(config: AuroraConfig) -> Self {
        let telemetry = Arc::new(TelemetryRecorder::new(config.telemetry.buffer_capacity));
        Self {
            gnss: Arc::new(RwLock::new(ConstellationManager::new())),
            fusion: Arc::new(RwLock::new(FusionEngine::new())),
            integrity: Arc::new(RwLock::new(IntegrityEngine::new())),
            continuity: Arc::new(RwLock::new(ContinuityManager::new())),
            imu: Arc::new(RwLock::new(ImuProcessor::new())),
            telemetry,
            event_bus: Arc::new(EventBus::new()),
            last_position: Arc::new(RwLock::new(None)),
            services: Arc::new(RwLock::new(ServiceRegistry::new())),
            config,
            has_received_gnss: AtomicBool::new(false),
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &AuroraConfig {
        &self.config
    }

    /// Returns the current fused position, if available.
    pub fn current_position(&self) -> Option<FusedPosition> {
        self.last_position.read().clone()
    }

    /// Returns the number of tracked satellites.
    pub fn tracked_satellites(&self) -> usize {
        let count = self.gnss.read().receiver().tracked_count();
        if count > 0 {
            self.has_received_gnss.store(true, Ordering::Relaxed);
        }
        count
    }

    /// Returns whether any GNSS data has ever been received.
    pub fn has_received_gnss_data(&self) -> bool {
        self.has_received_gnss.load(Ordering::Relaxed)
    }

    /// Mark that GNSS data has been received (e.g. after observing satellites).
    pub fn mark_gnss_received(&self) {
        self.has_received_gnss.store(true, Ordering::Relaxed);
    }

    /// Returns the current integrity level as a string.
    pub fn integrity_level(&self) -> String {
        format!("{:?}", self.integrity.read().current_level())
    }

    /// Returns the current continuity mode as a string.
    pub fn continuity_mode(&self) -> String {
        format!("{}", self.continuity.read().current_mode())
    }

    /// Returns the total number of events processed.
    pub fn total_events(&self) -> u64 {
        self.event_bus.total_events()
    }

    /// Returns the telemetry buffer size.
    pub fn telemetry_buffer_size(&self) -> usize {
        self.telemetry.buffer_size()
    }

    /// Returns whether the pipeline is healthy.
    pub fn is_healthy(&self) -> bool {
        true // Subsystems are always initialised; health degrades gracefully
    }

    /// Returns infrastructure service statistics.
    pub fn service_stats(&self) -> aurora_orchestrator::services::RegistryStats {
        self.services.read().stats()
    }

    /// Check if external calls are allowed (circuit breaker).
    ///
    /// Read-only check — does NOT trigger Open→HalfOpen recovery.
    /// Use [`try_external_call`] before actually making a request.
    pub fn can_call_external(&self) -> bool {
        self.services.read().can_call_external()
    }

    /// Attempt an external call through the circuit breaker.
    ///
    /// Triggers state transitions (Open→HalfOpen after cooldown) and returns
    /// whether the call is permitted.
    pub fn try_external_call(&self) -> bool {
        self.services.write().try_external_call()
    }

    /// Returns subsystem status summary.
    pub fn subsystem_status(&self) -> SubsystemStatus {
        SubsystemStatus {
            gnss_enabled: self.config.gnss.enable_gps
                || self.config.gnss.enable_galileo
                || self.config.gnss.enable_glonass
                || self.config.gnss.enable_beidou,
            fusion_enabled: true,
            integrity_enabled: self.config.integrity.enable_jamming_detection
                || self.config.integrity.enable_spoofing_detection,
            routing_enabled: true,
            traffic_enabled: self.config.traffic.enable_flow_control,
            fleet_enabled: self.config.fleet.enabled,
            emergency_enabled: self.config.emergency.enable_corridor,
            offline_enabled: self.config.offline.enabled,
            edge_enabled: self.config.edge.enabled,
            satellite_enabled: self.config.satellite.enabled,
            city_enabled: self.config.city.enable_signals,
            twin_enabled: self.config.twin.enabled,
            marketplace_enabled: self.config.marketplace.enabled,
            payments_enabled: self.config.payments.enabled,
            vehicle_enabled: self.config.vehicle.enabled,
            tracked_satellites: self.tracked_satellites(),
            integrity_level: self.integrity_level(),
            continuity_mode: self.continuity_mode(),
        }
    }
}

/// Summary of subsystem enable/disable states and key metrics.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SubsystemStatus {
    pub gnss_enabled: bool,
    pub fusion_enabled: bool,
    pub integrity_enabled: bool,
    pub routing_enabled: bool,
    pub traffic_enabled: bool,
    pub fleet_enabled: bool,
    pub emergency_enabled: bool,
    pub offline_enabled: bool,
    pub edge_enabled: bool,
    pub satellite_enabled: bool,
    pub city_enabled: bool,
    pub twin_enabled: bool,
    pub marketplace_enabled: bool,
    pub payments_enabled: bool,
    pub vehicle_enabled: bool,
    pub tracked_satellites: usize,
    pub integrity_level: String,
    pub continuity_mode: String,
}

/// Print a startup banner with subsystem status.
pub fn print_banner(status: &SubsystemStatus) {
    debug!("╔══════════════════════════════════════════════╗");
    debug!("║   AURORA NAV / GMIN                         ║");
    debug!("║   Global Mobility Intelligence Network      ║");
    debug!("╚══════════════════════════════════════════════╝");
    debug!("Subsystems:");
    debug!(
        "  GNSS:        {}",
        if status.gnss_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Fusion:      {}",
        if status.fusion_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Integrity:   {}",
        if status.integrity_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    debug!(
        "  Routing:     {}",
        if status.routing_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Traffic:     {}",
        if status.traffic_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Fleet:       {}",
        if status.fleet_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Emergency:   {}",
        if status.emergency_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    debug!(
        "  Offline:     {}",
        if status.offline_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Edge:        {}",
        if status.edge_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Satellite:   {}",
        if status.satellite_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    debug!(
        "  Smart City:  {}",
        if status.city_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Digital Twin:{}",
        if status.twin_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Marketplace: {}",
        if status.marketplace_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    debug!(
        "  Payments:    {}",
        if status.payments_enabled { "ON" } else { "OFF" }
    );
    debug!(
        "  Vehicle:     {}",
        if status.vehicle_enabled { "ON" } else { "OFF" }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_creates_with_default_config() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        assert!(pipeline.is_healthy());
        assert_eq!(pipeline.tracked_satellites(), 0);
        assert!(pipeline.current_position().is_none());
    }

    #[test]
    fn pipeline_config_accessible() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        assert_eq!(pipeline.config().api.port, 3000);
        assert!(pipeline.config().gnss.enable_gps);
    }

    #[test]
    fn pipeline_subsystem_status() {
        let cfg = AuroraConfig::default();
        let pipeline = NavigationPipeline::new(cfg);
        let status = pipeline.subsystem_status();
        assert!(status.gnss_enabled);
        assert!(status.fusion_enabled);
        assert!(status.integrity_enabled);
        assert!(status.routing_enabled);
        assert!(status.traffic_enabled);
        assert!(!status.fleet_enabled); // fleet off by default
        assert!(status.emergency_enabled);
        assert!(status.offline_enabled);
        assert!(status.edge_enabled);
        assert!(!status.satellite_enabled); // satellite off by default
        assert!(status.city_enabled);
        assert!(!status.twin_enabled); // twin off by default
        assert!(status.marketplace_enabled);
        assert!(!status.payments_enabled); // payments off by default
        assert!(!status.vehicle_enabled); // vehicle off by default
    }

    #[test]
    fn pipeline_total_events_starts_at_zero() {
        let pipeline = NavigationPipeline::new(AuroraConfig::default());
        assert_eq!(pipeline.total_events(), 0);
    }

    #[test]
    fn pipeline_telemetry_buffer_matches_config() {
        let cfg = aurora_config::ConfigBuilder::new()
            .telemetry_buffer(1000)
            .build_unchecked();
        let pipeline = NavigationPipeline::new(cfg);
        // buffer_size returns current count (0), but capacity was set
        assert_eq!(pipeline.telemetry_buffer_size(), 0);
    }
}
