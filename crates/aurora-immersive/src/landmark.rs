//! Landmark recognition — visual landmark database, matching, and
//! navigation anchoring using recognized landmarks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of visual landmark.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandmarkCategory {
    /// Notable building (monument, tower, etc.)
    Building,
    /// Bridge or overpass
    Bridge,
    /// Religious structure (mosque, church, synagogue)
    ReligiousStructure,
    /// Traffic landmark (roundabout, junction, toll gate)
    TrafficLandmark,
    /// Natural landmark (mountain, river, lake)
    Natural,
    /// Commercial (mall, gas station, restaurant chain)
    Commercial,
    /// Public infrastructure (hospital, school, government)
    PublicInfra,
    /// Art installation or statue
    ArtInstallation,
    /// Custom category
    Custom(String),
}

/// A visual landmark entry in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landmark {
    /// Unique identifier
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// Category
    pub category: LandmarkCategory,
    /// Latitude (WGS-84)
    pub lat: f64,
    /// Longitude (WGS-84)
    pub lon: f64,
    /// Estimated visibility radius (meters) — how far away it can be seen
    pub visibility_radius_m: f64,
    /// Bearing from which the landmark is most recognizable (degrees, 0-360)
    pub best_bearing_deg: Option<f32>,
    /// Description for navigation instructions
    pub nav_description: String,
    /// Visual feature descriptor hash (for matching)
    pub feature_hash: Option<String>,
    /// Confidence threshold for recognition (0.0 to 1.0)
    pub recognition_threshold: f32,
    /// Whether this landmark is usable at night
    pub night_visible: bool,
    /// Last verified timestamp
    pub last_verified: Option<DateTime<Utc>>,
}

impl Landmark {
    /// Create a new landmark.
    pub fn new(
        name: &str,
        category: LandmarkCategory,
        lat: f64,
        lon: f64,
        nav_description: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            category,
            lat,
            lon,
            visibility_radius_m: 500.0,
            best_bearing_deg: None,
            nav_description: nav_description.to_string(),
            feature_hash: None,
            recognition_threshold: 0.7,
            night_visible: false,
            last_verified: None,
        }
    }
}

/// Result of a landmark recognition attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    /// Matched landmark ID
    pub landmark_id: Uuid,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Estimated bearing to the landmark (degrees)
    pub bearing_deg: f32,
    /// Estimated distance to the landmark (meters)
    pub distance_m: f64,
    /// Timestamp of recognition
    pub timestamp: DateTime<Utc>,
}

/// Landmark database with spatial lookup.
#[derive(Debug)]
pub struct LandmarkDb {
    landmarks: Vec<Landmark>,
}

impl LandmarkDb {
    /// Create an empty database.
    pub fn new() -> Self {
        Self {
            landmarks: Vec::new(),
        }
    }

    /// Add a landmark to the database.
    pub fn add(&mut self, landmark: Landmark) {
        self.landmarks.push(landmark);
    }

    /// Find landmarks within a radius (meters) of a point.
    pub fn find_near(&self, lat: f64, lon: f64, radius_m: f64) -> Vec<&Landmark> {
        self.landmarks
            .iter()
            .filter(|lm| {
                let dist = haversine_m(lat, lon, lm.lat, lm.lon);
                dist <= radius_m
            })
            .collect()
    }

    /// Find landmarks visible from a given position and bearing.
    pub fn find_visible(
        &self,
        lat: f64,
        lon: f64,
        bearing_deg: f32,
        fov_deg: f32,
    ) -> Vec<&Landmark> {
        self.landmarks
            .iter()
            .filter(|lm| {
                let dist = haversine_m(lat, lon, lm.lat, lm.lon);
                if dist > lm.visibility_radius_m {
                    return false;
                }
                // Check if landmark is within the field of view
                let lm_bearing = bearing_to(lat, lon, lm.lat, lm.lon);
                let diff = angle_diff(bearing_deg, lm_bearing);
                diff <= fov_deg / 2.0
            })
            .collect()
    }

    /// Get a landmark by ID.
    pub fn get(&self, id: Uuid) -> Option<&Landmark> {
        self.landmarks.iter().find(|lm| lm.id == id)
    }

    /// Total landmark count.
    pub fn count(&self) -> usize {
        self.landmarks.len()
    }

    /// Find landmarks by category.
    pub fn find_by_category(&self, category: &LandmarkCategory) -> Vec<&Landmark> {
        self.landmarks
            .iter()
            .filter(|lm| &lm.category == category)
            .collect()
    }
}

impl Default for LandmarkDb {
    fn default() -> Self {
        Self::new()
    }
}

/// Haversine distance in meters.
fn haversine_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    R * 2.0 * a.sqrt().asin()
}

/// Bearing from point 1 to point 2 in degrees (0-360).
fn bearing_to(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f32 {
    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let bearing = y.atan2(x).to_degrees();
    ((bearing + 360.0) % 360.0) as f32
}

/// Absolute angular difference between two bearings in degrees.
fn angle_diff(a: f32, b: f32) -> f32 {
    let diff = (a - b).abs() % 360.0;
    if diff > 180.0 {
        360.0 - diff
    } else {
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landmark_creation() {
        let lm = Landmark::new(
            "Azrieli Tower",
            LandmarkCategory::Building,
            32.0741,
            34.7922,
            "tall triangular tower on your right",
        );
        assert_eq!(lm.name, "Azrieli Tower");
        assert!(!lm.night_visible);
    }

    #[test]
    fn test_landmark_db_find_near() {
        let mut db = LandmarkDb::new();
        db.add(Landmark::new(
            "Tower A",
            LandmarkCategory::Building,
            32.07,
            34.79,
            "tower",
        ));
        db.add(Landmark::new(
            "Tower B",
            LandmarkCategory::Building,
            33.0,
            35.0,
            "far tower",
        ));

        let near = db.find_near(32.07, 34.79, 1000.0);
        assert_eq!(near.len(), 1);
        assert_eq!(near[0].name, "Tower A");
    }

    #[test]
    fn test_landmark_db_find_by_category() {
        let mut db = LandmarkDb::new();
        db.add(Landmark::new(
            "Bridge",
            LandmarkCategory::Bridge,
            32.0,
            34.0,
            "bridge",
        ));
        db.add(Landmark::new(
            "Tower",
            LandmarkCategory::Building,
            32.0,
            34.0,
            "tower",
        ));

        assert_eq!(db.find_by_category(&LandmarkCategory::Bridge).len(), 1);
        assert_eq!(db.find_by_category(&LandmarkCategory::Building).len(), 1);
    }

    #[test]
    fn test_angle_diff() {
        assert!((angle_diff(10.0, 350.0) - 20.0).abs() < 0.01);
        assert!((angle_diff(0.0, 180.0) - 180.0).abs() < 0.01);
        assert!((angle_diff(90.0, 90.0)).abs() < 0.01);
    }

    #[test]
    fn test_bearing_to() {
        // Due east should be approximately 90 degrees
        let b = bearing_to(0.0, 0.0, 0.0, 1.0);
        assert!((b - 90.0).abs() < 1.0);
    }

    #[test]
    fn test_find_visible_within_fov() {
        let mut db = LandmarkDb::new();
        let mut lm = Landmark::new("Test", LandmarkCategory::Building, 32.071, 34.79, "test");
        lm.visibility_radius_m = 2000.0;
        db.add(lm);

        // Looking roughly north from south of the landmark
        let visible = db.find_visible(32.07, 34.79, 0.0, 90.0);
        assert_eq!(visible.len(), 1);
    }
}
