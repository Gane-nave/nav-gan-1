//! Proximity alerts — warn users about nearby hazards, POIs, and conditions.

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Alert severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Danger,
    Emergency,
}

/// Type of proximity alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertType {
    SpeedCamera,
    Accident,
    Construction,
    Weather,
    SchoolZone,
    RailCrossing,
    SharpCurve,
    SteepGrade,
    LowBridge,
    Custom,
}

/// A proximity alert definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityAlert {
    pub id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub lat: f64,
    pub lon: f64,
    pub radius_m: f64,
    pub heading_range: Option<(f64, f64)>,
    pub message: String,
    pub active: bool,
    pub valid_until: Option<DateTime<Utc>>,
}

/// A triggered alert event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub alert_id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub distance_m: f64,
    pub timestamp: DateTime<Utc>,
}

/// Proximity alert engine — checks position against registered alerts.
pub struct AlertEngine {
    alerts: RwLock<Vec<ProximityAlert>>,
    recently_triggered: RwLock<std::collections::HashMap<Uuid, DateTime<Utc>>>,
    suppression_secs: i64,
}

impl AlertEngine {
    /// Create a new engine with a suppression window (seconds between re-triggers).
    pub fn new(suppression_secs: i64) -> Self {
        Self {
            alerts: RwLock::new(Vec::new()),
            recently_triggered: RwLock::new(std::collections::HashMap::new()),
            suppression_secs,
        }
    }

    /// Register an alert.
    pub fn add_alert(&self, alert: ProximityAlert) {
        self.alerts.write().push(alert);
    }

    /// Remove an alert by ID.
    pub fn remove_alert(&self, id: Uuid) -> bool {
        let mut alerts = self.alerts.write();
        let before = alerts.len();
        alerts.retain(|a| a.id != id);
        alerts.len() < before
    }

    /// Calculate haversine distance in meters.
    fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let dlat = (lat2 - lat1).to_radians();
        let dlon = (lon2 - lon1).to_radians();
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
        2.0 * 6_371_000.0 * a.sqrt().asin()
    }

    /// Check position against all alerts. Returns triggered events sorted by severity.
    pub fn check(
        &self,
        lat: f64,
        lon: f64,
        heading: Option<f64>,
        now: DateTime<Utc>,
    ) -> Vec<AlertEvent> {
        let alerts = self.alerts.read();
        let mut triggered = self.recently_triggered.write();
        let mut events = Vec::new();

        for alert in alerts.iter() {
            if !alert.active {
                continue;
            }
            if let Some(valid_until) = alert.valid_until {
                if now > valid_until {
                    continue;
                }
            }

            // Check suppression
            if let Some(last) = triggered.get(&alert.id) {
                let elapsed = (now - *last).num_seconds();
                if elapsed >= 0 && elapsed < self.suppression_secs {
                    continue;
                }
            }

            let dist = Self::haversine(lat, lon, alert.lat, alert.lon);
            if dist > alert.radius_m {
                continue;
            }

            // Check heading range if specified
            if let (Some((min_h, max_h)), Some(h)) = (alert.heading_range, heading) {
                if min_h <= max_h {
                    if h < min_h || h > max_h {
                        continue;
                    }
                } else {
                    // Wraps around 360
                    if h < min_h && h > max_h {
                        continue;
                    }
                }
            }

            events.push(AlertEvent {
                alert_id: alert.id,
                alert_type: alert.alert_type,
                severity: alert.severity,
                message: alert.message.clone(),
                distance_m: dist,
                timestamp: now,
            });
            triggered.insert(alert.id, now);
        }

        // Sort by severity (highest first), then by distance
        events.sort_by(|a, b| {
            b.severity.cmp(&a.severity).then_with(|| {
                a.distance_m
                    .partial_cmp(&b.distance_m)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });
        events
    }

    /// Count active alerts.
    pub fn active_count(&self) -> usize {
        self.alerts.read().iter().filter(|a| a.active).count()
    }

    /// Purge expired alerts.
    pub fn purge_expired(&self, now: DateTime<Utc>) -> usize {
        let mut alerts = self.alerts.write();
        let before = alerts.len();
        alerts.retain(|a| a.valid_until.map_or(true, |vu| now <= vu));
        before - alerts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_alert(lat: f64, lon: f64, radius: f64, severity: AlertSeverity) -> ProximityAlert {
        ProximityAlert {
            id: Uuid::new_v4(),
            alert_type: AlertType::SpeedCamera,
            severity,
            lat,
            lon,
            radius_m: radius,
            heading_range: None,
            message: "Alert".to_string(),
            active: true,
            valid_until: None,
        }
    }

    #[test]
    fn test_proximity_trigger() {
        let engine = AlertEngine::new(0);
        engine.add_alert(make_alert(32.0, 34.0, 1000.0, AlertSeverity::Warning));
        let now = Utc::now();
        let events = engine.check(32.0001, 34.0001, None, now);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].severity, AlertSeverity::Warning);
    }

    #[test]
    fn test_out_of_range() {
        let engine = AlertEngine::new(0);
        engine.add_alert(make_alert(32.0, 34.0, 100.0, AlertSeverity::Warning));
        let events = engine.check(33.0, 35.0, None, Utc::now());
        assert!(events.is_empty());
    }

    #[test]
    fn test_suppression() {
        let engine = AlertEngine::new(60);
        engine.add_alert(make_alert(32.0, 34.0, 1000.0, AlertSeverity::Warning));
        let now = Utc::now();
        let e1 = engine.check(32.0, 34.0, None, now);
        assert_eq!(e1.len(), 1);
        // Within suppression window
        let e2 = engine.check(32.0, 34.0, None, now + chrono::Duration::seconds(30));
        assert!(e2.is_empty());
        // After suppression
        let e3 = engine.check(32.0, 34.0, None, now + chrono::Duration::seconds(61));
        assert_eq!(e3.len(), 1);
    }

    #[test]
    fn test_inactive_ignored() {
        let engine = AlertEngine::new(0);
        let mut alert = make_alert(32.0, 34.0, 1000.0, AlertSeverity::Danger);
        alert.active = false;
        engine.add_alert(alert);
        let events = engine.check(32.0, 34.0, None, Utc::now());
        assert!(events.is_empty());
    }

    #[test]
    fn test_expired_alert() {
        let engine = AlertEngine::new(0);
        let mut alert = make_alert(32.0, 34.0, 1000.0, AlertSeverity::Info);
        alert.valid_until = Some(Utc::now() - chrono::Duration::hours(1));
        engine.add_alert(alert);
        let events = engine.check(32.0, 34.0, None, Utc::now());
        assert!(events.is_empty());
    }

    #[test]
    fn test_severity_sorting() {
        let engine = AlertEngine::new(0);
        engine.add_alert(make_alert(32.0, 34.0, 5000.0, AlertSeverity::Info));
        engine.add_alert(make_alert(32.0, 34.0, 5000.0, AlertSeverity::Emergency));
        engine.add_alert(make_alert(32.0, 34.0, 5000.0, AlertSeverity::Warning));
        let events = engine.check(32.0, 34.0, None, Utc::now());
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].severity, AlertSeverity::Emergency);
        assert_eq!(events[1].severity, AlertSeverity::Warning);
        assert_eq!(events[2].severity, AlertSeverity::Info);
    }

    #[test]
    fn test_heading_filter() {
        let engine = AlertEngine::new(0);
        let mut alert = make_alert(32.0, 34.0, 5000.0, AlertSeverity::Warning);
        alert.heading_range = Some((350.0, 10.0)); // wraps around 360
        engine.add_alert(alert);
        // Heading 0 (north) — within range
        let e1 = engine.check(32.0, 34.0, Some(0.0), Utc::now());
        assert_eq!(e1.len(), 1);
    }

    #[test]
    fn test_heading_out_of_range() {
        let engine = AlertEngine::new(0);
        let mut alert = make_alert(32.0, 34.0, 5000.0, AlertSeverity::Warning);
        alert.heading_range = Some((80.0, 100.0)); // east
        engine.add_alert(alert);
        // Heading 180 (south) — out of range
        let e = engine.check(32.0, 34.0, Some(180.0), Utc::now());
        assert!(e.is_empty());
    }

    #[test]
    fn test_purge_expired() {
        let engine = AlertEngine::new(0);
        engine.add_alert(make_alert(32.0, 34.0, 1000.0, AlertSeverity::Info));
        let mut expired = make_alert(32.0, 34.0, 1000.0, AlertSeverity::Warning);
        expired.valid_until = Some(Utc::now() - chrono::Duration::hours(1));
        engine.add_alert(expired);
        let purged = engine.purge_expired(Utc::now());
        assert_eq!(purged, 1);
        assert_eq!(engine.active_count(), 1);
    }

    #[test]
    fn test_remove_alert() {
        let engine = AlertEngine::new(0);
        let alert = make_alert(32.0, 34.0, 1000.0, AlertSeverity::Info);
        let id = alert.id;
        engine.add_alert(alert);
        assert!(engine.remove_alert(id));
        assert!(!engine.remove_alert(id));
    }
}
