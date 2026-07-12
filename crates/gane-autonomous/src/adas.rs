//! Advanced Driver Assistance Systems (ADAS) integration layer.

use std::time::Instant;

/// ADAS feature availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdasFeature {
    /// Lane departure warning.
    LaneDepartureWarning,
    /// Forward collision warning.
    ForwardCollisionWarning,
    /// Blind spot monitoring.
    BlindSpotMonitoring,
    /// Automatic emergency braking.
    AutoEmergencyBraking,
    /// Traffic sign recognition.
    TrafficSignRecognition,
    /// Pedestrian detection.
    PedestrianDetection,
}

/// ADAS alert severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdasAlertLevel {
    /// Informational — no action required.
    Info,
    /// Warning — driver attention needed.
    Warning,
    /// Critical — immediate action required.
    Critical,
    /// Emergency — system intervening.
    Emergency,
}

/// An ADAS alert event.
#[derive(Debug, Clone)]
pub struct AdasAlert {
    /// Feature that triggered the alert.
    pub feature: AdasFeature,
    /// Severity level.
    pub level: AdasAlertLevel,
    /// Distance to hazard (metres), if applicable.
    pub distance_m: Option<f64>,
    /// Time to collision (seconds), if applicable.
    pub ttc_secs: Option<f64>,
    /// Description.
    pub description: String,
    /// Timestamp.
    pub timestamp: Instant,
}

/// ADAS sensor status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorStatus {
    /// Sensor is operational.
    Operational,
    /// Sensor is degraded (e.g., dirty lens).
    Degraded,
    /// Sensor has failed.
    Failed,
    /// Sensor is not available on this vehicle.
    NotAvailable,
}

/// ADAS configuration.
#[derive(Debug, Clone)]
pub struct AdasConfig {
    /// Minimum time-to-collision for forward collision warning (seconds).
    pub fcw_ttc_threshold: f64,
    /// Lane departure warning sensitivity (0.0 = off, 1.0 = max).
    pub ldw_sensitivity: f64,
    /// Enable automatic emergency braking.
    pub aeb_enabled: bool,
    /// Blind spot warning distance (metres).
    pub blind_spot_distance: f64,
}

impl Default for AdasConfig {
    fn default() -> Self {
        Self {
            fcw_ttc_threshold: 2.5,
            ldw_sensitivity: 0.7,
            aeb_enabled: true,
            blind_spot_distance: 5.0,
        }
    }
}

/// ADAS integration manager.
pub struct AdasManager {
    config: AdasConfig,
    sensors: Vec<(AdasFeature, SensorStatus)>,
    alerts: Vec<AdasAlert>,
    max_alerts: usize,
}

impl AdasManager {
    /// Create a new ADAS manager.
    pub fn new(config: AdasConfig) -> Self {
        Self {
            config,
            sensors: Vec::new(),
            alerts: Vec::new(),
            max_alerts: 100,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AdasConfig::default())
    }

    /// Register a sensor.
    pub fn register_sensor(&mut self, feature: AdasFeature, status: SensorStatus) {
        // Update if already registered
        if let Some(entry) = self.sensors.iter_mut().find(|(f, _)| *f == feature) {
            entry.1 = status;
        } else {
            self.sensors.push((feature, status));
        }
    }

    /// Get sensor status for a feature.
    pub fn sensor_status(&self, feature: AdasFeature) -> SensorStatus {
        self.sensors
            .iter()
            .find(|(f, _)| *f == feature)
            .map(|(_, s)| *s)
            .unwrap_or(SensorStatus::NotAvailable)
    }

    /// Check if a feature is operational.
    pub fn is_feature_available(&self, feature: AdasFeature) -> bool {
        self.sensor_status(feature) == SensorStatus::Operational
    }

    /// Process a forward collision event.
    pub fn check_forward_collision(
        &mut self,
        distance_m: f64,
        relative_speed_mps: f64,
    ) -> Option<AdasAlert> {
        if !self.is_feature_available(AdasFeature::ForwardCollisionWarning) {
            return None;
        }

        if relative_speed_mps <= 0.0 {
            return None; // Not closing
        }

        let ttc = distance_m / relative_speed_mps;

        let (level, description) = if ttc < 1.0 && self.config.aeb_enabled {
            (
                AdasAlertLevel::Emergency,
                format!("AEB ACTIVE — collision in {ttc:.1}s"),
            )
        } else if ttc < self.config.fcw_ttc_threshold * 0.5 {
            (
                AdasAlertLevel::Critical,
                format!("Imminent collision — {ttc:.1}s at {distance_m:.0}m"),
            )
        } else if ttc < self.config.fcw_ttc_threshold {
            (
                AdasAlertLevel::Warning,
                format!("Forward collision warning — {ttc:.1}s at {distance_m:.0}m"),
            )
        } else {
            return None;
        };

        let alert = AdasAlert {
            feature: AdasFeature::ForwardCollisionWarning,
            level,
            distance_m: Some(distance_m),
            ttc_secs: Some(ttc),
            description,
            timestamp: Instant::now(),
        };
        self.record_alert(alert.clone());
        Some(alert)
    }

    /// Check for lane departure.
    pub fn check_lane_departure(
        &mut self,
        deviation_m: f64,
        lane_width_m: f64,
    ) -> Option<AdasAlert> {
        if !self.is_feature_available(AdasFeature::LaneDepartureWarning) {
            return None;
        }

        let threshold = (lane_width_m / 2.0) * (1.0 - self.config.ldw_sensitivity);
        let abs_deviation = deviation_m.abs();

        if abs_deviation < threshold {
            return None;
        }

        let level = if abs_deviation > lane_width_m * 0.45 {
            AdasAlertLevel::Critical
        } else {
            AdasAlertLevel::Warning
        };

        let direction = if deviation_m > 0.0 { "right" } else { "left" };
        let alert = AdasAlert {
            feature: AdasFeature::LaneDepartureWarning,
            level,
            distance_m: Some(abs_deviation),
            ttc_secs: None,
            description: format!("Lane departure {direction} — {abs_deviation:.2}m from centre"),
            timestamp: Instant::now(),
        };
        self.record_alert(alert.clone());
        Some(alert)
    }

    /// Check blind spot.
    pub fn check_blind_spot(&mut self, object_distance_m: f64, is_left: bool) -> Option<AdasAlert> {
        if !self.is_feature_available(AdasFeature::BlindSpotMonitoring) {
            return None;
        }

        if object_distance_m > self.config.blind_spot_distance {
            return None;
        }

        let side = if is_left { "left" } else { "right" };
        let alert = AdasAlert {
            feature: AdasFeature::BlindSpotMonitoring,
            level: AdasAlertLevel::Warning,
            distance_m: Some(object_distance_m),
            ttc_secs: None,
            description: format!("Vehicle in {side} blind spot at {object_distance_m:.1}m"),
            timestamp: Instant::now(),
        };
        self.record_alert(alert.clone());
        Some(alert)
    }

    /// Get the number of alerts recorded.
    pub fn alert_count(&self) -> usize {
        self.alerts.len()
    }

    fn record_alert(&mut self, alert: AdasAlert) {
        self.alerts.push(alert);
        if self.alerts.len() > self.max_alerts {
            self.alerts.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_manager() -> AdasManager {
        let mut mgr = AdasManager::with_defaults();
        mgr.register_sensor(
            AdasFeature::ForwardCollisionWarning,
            SensorStatus::Operational,
        );
        mgr.register_sensor(AdasFeature::LaneDepartureWarning, SensorStatus::Operational);
        mgr.register_sensor(AdasFeature::BlindSpotMonitoring, SensorStatus::Operational);
        mgr.register_sensor(AdasFeature::AutoEmergencyBraking, SensorStatus::Operational);
        mgr
    }

    #[test]
    fn test_no_alert_when_safe() {
        let mut mgr = setup_manager();
        // Far away, slow approach
        let alert = mgr.check_forward_collision(100.0, 5.0); // TTC = 20s
        assert!(alert.is_none());
    }

    #[test]
    fn test_fcw_warning() {
        let mut mgr = setup_manager();
        // TTC = 2.0s, below threshold (2.5)
        let alert = mgr.check_forward_collision(20.0, 10.0);
        assert!(alert.is_some());
        let alert = alert.unwrap();
        assert_eq!(alert.level, AdasAlertLevel::Warning);
    }

    #[test]
    fn test_fcw_critical() {
        let mut mgr = setup_manager();
        // TTC = 1.0s, below 50% of threshold
        let alert = mgr.check_forward_collision(10.0, 10.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().level, AdasAlertLevel::Critical);
    }

    #[test]
    fn test_aeb_emergency() {
        let mut mgr = setup_manager();
        // TTC = 0.5s, AEB triggers
        let alert = mgr.check_forward_collision(5.0, 10.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().level, AdasAlertLevel::Emergency);
    }

    #[test]
    fn test_no_alert_when_not_closing() {
        let mut mgr = setup_manager();
        // Negative relative speed = moving apart
        let alert = mgr.check_forward_collision(10.0, -5.0);
        assert!(alert.is_none());
    }

    #[test]
    fn test_feature_not_available() {
        let mut mgr = AdasManager::with_defaults();
        // No sensors registered
        let alert = mgr.check_forward_collision(5.0, 10.0);
        assert!(alert.is_none());
    }

    #[test]
    fn test_degraded_sensor_not_operational() {
        let mut mgr = AdasManager::with_defaults();
        mgr.register_sensor(AdasFeature::ForwardCollisionWarning, SensorStatus::Degraded);
        assert!(!mgr.is_feature_available(AdasFeature::ForwardCollisionWarning));
    }

    #[test]
    fn test_lane_departure_warning() {
        let mut mgr = setup_manager();
        // Deviation 1.2m from centre in 3.5m lane
        let alert = mgr.check_lane_departure(1.2, 3.5);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().feature, AdasFeature::LaneDepartureWarning);
    }

    #[test]
    fn test_lane_departure_small_deviation_no_alert() {
        let mut mgr = setup_manager();
        // Small deviation within sensitivity range
        let alert = mgr.check_lane_departure(0.3, 3.5);
        assert!(alert.is_none());
    }

    #[test]
    fn test_blind_spot_warning() {
        let mut mgr = setup_manager();
        let alert = mgr.check_blind_spot(3.0, true);
        assert!(alert.is_some());
        let alert = alert.unwrap();
        assert_eq!(alert.feature, AdasFeature::BlindSpotMonitoring);
        assert!(alert.description.contains("left"));
    }

    #[test]
    fn test_blind_spot_out_of_range() {
        let mut mgr = setup_manager();
        let alert = mgr.check_blind_spot(10.0, false); // Beyond 5m threshold
        assert!(alert.is_none());
    }

    #[test]
    fn test_alert_count() {
        let mut mgr = setup_manager();
        mgr.check_forward_collision(5.0, 10.0);
        mgr.check_lane_departure(1.5, 3.5);
        mgr.check_blind_spot(3.0, true);
        assert_eq!(mgr.alert_count(), 3);
    }

    #[test]
    fn test_sensor_status_update() {
        let mut mgr = AdasManager::with_defaults();
        mgr.register_sensor(
            AdasFeature::ForwardCollisionWarning,
            SensorStatus::Operational,
        );
        assert_eq!(
            mgr.sensor_status(AdasFeature::ForwardCollisionWarning),
            SensorStatus::Operational
        );
        mgr.register_sensor(AdasFeature::ForwardCollisionWarning, SensorStatus::Failed);
        assert_eq!(
            mgr.sensor_status(AdasFeature::ForwardCollisionWarning),
            SensorStatus::Failed
        );
    }
}
