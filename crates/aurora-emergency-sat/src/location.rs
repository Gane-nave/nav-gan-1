//! Emergency location sharing — position broadcasting, location tracking,
//! and rescue coordination with multi-source position fusion.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Source of location information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocationSource {
    /// GNSS fix (GPS/GLONASS/Galileo/BeiDou)
    Gnss,
    /// Cell tower triangulation
    CellTower,
    /// Wi-Fi positioning
    WiFi,
    /// Manual entry by user
    Manual,
    /// Received from satellite relay
    SatRelay,
    /// Dead reckoning (IMU-based)
    DeadReckoning,
    /// Last known position (stale)
    LastKnown,
}

/// A single emergency location report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyLocation {
    /// Report identifier
    pub id: Uuid,
    /// Device that reported the location
    pub device_id: Uuid,
    /// Latitude (WGS-84)
    pub lat: f64,
    /// Longitude (WGS-84)
    pub lon: f64,
    /// Altitude MSL (meters)
    pub alt_m: Option<f32>,
    /// Horizontal accuracy (meters, 95% confidence)
    pub h_accuracy_m: f32,
    /// Vertical accuracy (meters, 95% confidence)
    pub v_accuracy_m: Option<f32>,
    /// Speed (m/s)
    pub speed_mps: Option<f32>,
    /// Heading (degrees, 0-360)
    pub heading_deg: Option<f32>,
    /// Source of this fix
    pub source: LocationSource,
    /// Timestamp of the fix
    pub timestamp: DateTime<Utc>,
    /// Battery level (0.0 to 1.0)
    pub battery_level: Option<f32>,
    /// Whether the device is in motion
    pub in_motion: bool,
}

impl EmergencyLocation {
    /// Create a new emergency location report.
    pub fn new(
        device_id: Uuid,
        lat: f64,
        lon: f64,
        h_accuracy_m: f32,
        source: LocationSource,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            device_id,
            lat,
            lon,
            alt_m: None,
            h_accuracy_m,
            v_accuracy_m: None,
            speed_mps: None,
            heading_deg: None,
            source,
            timestamp: Utc::now(),
            battery_level: None,
            in_motion: false,
        }
    }

    /// Age of the fix in seconds.
    pub fn age_seconds(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.timestamp)
            .num_seconds()
    }

    /// Whether this fix is considered stale (older than threshold seconds).
    pub fn is_stale(&self, threshold_s: i64) -> bool {
        self.age_seconds() > threshold_s
    }

    /// Haversine distance to another location in meters.
    pub fn distance_to(&self, other: &EmergencyLocation) -> f64 {
        haversine_distance_m(self.lat, self.lon, other.lat, other.lon)
    }
}

/// Compute haversine distance between two WGS-84 points in meters.
pub fn haversine_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_M: f64 = 6_371_000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    EARTH_RADIUS_M * c
}

/// Emergency location tracker — maintains current positions for multiple
/// devices and supports rescue coordination queries.
#[derive(Debug)]
pub struct LocationTracker {
    /// Latest location per device
    locations: HashMap<Uuid, EmergencyLocation>,
    /// Location history per device (newest first, capped)
    history: HashMap<Uuid, Vec<EmergencyLocation>>,
    /// Maximum history entries per device
    max_history: usize,
    /// Stale threshold in seconds
    stale_threshold_s: i64,
}

impl LocationTracker {
    /// Create a new tracker.
    pub fn new(max_history: usize, stale_threshold_s: i64) -> Self {
        Self {
            locations: HashMap::new(),
            history: HashMap::new(),
            max_history,
            stale_threshold_s,
        }
    }

    /// Update a device's location.
    pub fn update(&mut self, loc: EmergencyLocation) {
        let device_id = loc.device_id;
        // Push old location to history
        if let Some(old) = self.locations.get(&device_id) {
            let hist = self.history.entry(device_id).or_default();
            hist.insert(0, old.clone());
            if hist.len() > self.max_history {
                hist.truncate(self.max_history);
            }
        }
        self.locations.insert(device_id, loc);
    }

    /// Get the latest location for a device.
    pub fn latest(&self, device_id: Uuid) -> Option<&EmergencyLocation> {
        self.locations.get(&device_id)
    }

    /// Get location history for a device.
    pub fn history(&self, device_id: Uuid) -> &[EmergencyLocation] {
        self.history
            .get(&device_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Find all devices within a radius (meters) of a point.
    pub fn devices_near(&self, lat: f64, lon: f64, radius_m: f64) -> Vec<&EmergencyLocation> {
        self.locations
            .values()
            .filter(|loc| {
                let dist = haversine_distance_m(lat, lon, loc.lat, loc.lon);
                dist <= radius_m
            })
            .collect()
    }

    /// Get all active (non-stale) device locations.
    pub fn active_devices(&self) -> Vec<&EmergencyLocation> {
        self.locations
            .values()
            .filter(|loc| !loc.is_stale(self.stale_threshold_s))
            .collect()
    }

    /// Get all stale device locations.
    pub fn stale_devices(&self) -> Vec<&EmergencyLocation> {
        self.locations
            .values()
            .filter(|loc| loc.is_stale(self.stale_threshold_s))
            .collect()
    }

    /// Number of tracked devices.
    pub fn device_count(&self) -> usize {
        self.locations.len()
    }

    /// Remove all stale entries and return the count of removed devices.
    pub fn prune_stale(&mut self) -> usize {
        let stale_ids: Vec<Uuid> = self
            .locations
            .iter()
            .filter(|(_, loc)| loc.is_stale(self.stale_threshold_s))
            .map(|(id, _)| *id)
            .collect();
        let count = stale_ids.len();
        for id in stale_ids {
            self.locations.remove(&id);
            self.history.remove(&id);
        }
        count
    }

    /// Compute the bounding box of all active device locations.
    /// Returns `(min_lat, min_lon, max_lat, max_lon)` or `None` if no active devices.
    pub fn bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        let active = self.active_devices();
        if active.is_empty() {
            return None;
        }
        let mut min_lat = f64::MAX;
        let mut min_lon = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut max_lon = f64::MIN;
        for loc in &active {
            min_lat = min_lat.min(loc.lat);
            min_lon = min_lon.min(loc.lon);
            max_lat = max_lat.max(loc.lat);
            max_lon = max_lon.max(loc.lon);
        }
        Some((min_lat, min_lon, max_lat, max_lon))
    }
}

impl Default for LocationTracker {
    fn default() -> Self {
        Self::new(100, 300) // 100 history entries, 5 min stale threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_creation() {
        let device = Uuid::new_v4();
        let loc = EmergencyLocation::new(device, 32.08, 34.78, 5.0, LocationSource::Gnss);
        assert_eq!(loc.device_id, device);
        assert!(!loc.is_stale(300));
        assert!((loc.h_accuracy_m - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_haversine_distance() {
        // Tel Aviv to Jerusalem ≈ 54 km
        let dist = haversine_distance_m(32.0853, 34.7818, 31.7683, 35.2137);
        assert!((dist - 54_000.0).abs() < 2000.0); // Within 2 km tolerance
    }

    #[test]
    fn test_haversine_same_point() {
        let dist = haversine_distance_m(32.0, 34.0, 32.0, 34.0);
        assert!(dist < 0.01);
    }

    #[test]
    fn test_tracker_update_and_latest() {
        let mut tracker = LocationTracker::default();
        let device = Uuid::new_v4();
        let loc = EmergencyLocation::new(device, 32.08, 34.78, 5.0, LocationSource::Gnss);
        tracker.update(loc);

        let latest = tracker.latest(device).unwrap();
        assert!((latest.lat - 32.08).abs() < f64::EPSILON);
        assert_eq!(tracker.device_count(), 1);
    }

    #[test]
    fn test_tracker_history() {
        let mut tracker = LocationTracker::new(10, 300);
        let device = Uuid::new_v4();

        // First update
        let loc1 = EmergencyLocation::new(device, 32.08, 34.78, 5.0, LocationSource::Gnss);
        tracker.update(loc1);

        // Second update pushes first to history
        let loc2 = EmergencyLocation::new(device, 32.09, 34.79, 3.0, LocationSource::Gnss);
        tracker.update(loc2);

        assert_eq!(tracker.history(device).len(), 1);
        let latest = tracker.latest(device).unwrap();
        assert!((latest.lat - 32.09).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tracker_devices_near() {
        let mut tracker = LocationTracker::default();
        let d1 = Uuid::new_v4();
        let d2 = Uuid::new_v4();

        tracker.update(EmergencyLocation::new(
            d1,
            32.08,
            34.78,
            5.0,
            LocationSource::Gnss,
        ));
        tracker.update(EmergencyLocation::new(
            d2,
            33.0,
            35.0,
            5.0,
            LocationSource::Gnss,
        ));

        // Search near first device (1 km radius)
        let near = tracker.devices_near(32.08, 34.78, 1000.0);
        assert_eq!(near.len(), 1);

        // Search with large radius
        let near_all = tracker.devices_near(32.08, 34.78, 200_000.0);
        assert_eq!(near_all.len(), 2);
    }

    #[test]
    fn test_tracker_bounding_box() {
        let mut tracker = LocationTracker::default();
        let d1 = Uuid::new_v4();
        let d2 = Uuid::new_v4();

        tracker.update(EmergencyLocation::new(
            d1,
            31.0,
            34.0,
            5.0,
            LocationSource::Gnss,
        ));
        tracker.update(EmergencyLocation::new(
            d2,
            33.0,
            36.0,
            5.0,
            LocationSource::Gnss,
        ));

        let bb = tracker.bounding_box().unwrap();
        assert!((bb.0 - 31.0).abs() < f64::EPSILON); // min_lat
        assert!((bb.1 - 34.0).abs() < f64::EPSILON); // min_lon
        assert!((bb.2 - 33.0).abs() < f64::EPSILON); // max_lat
        assert!((bb.3 - 36.0).abs() < f64::EPSILON); // max_lon
    }

    #[test]
    fn test_location_distance_to() {
        let d = Uuid::new_v4();
        let loc1 = EmergencyLocation::new(d, 32.08, 34.78, 5.0, LocationSource::Gnss);
        let loc2 = EmergencyLocation::new(d, 32.08, 34.78, 5.0, LocationSource::Gnss);
        assert!(loc1.distance_to(&loc2) < 1.0);
    }
}
