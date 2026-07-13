//! Test fixtures — reusable test data generators and scenario builders.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A test fixture — reusable test data with setup/teardown.
#[derive(Debug, Clone)]
pub struct Fixture<T: Clone> {
    name: String,
    data: T,
    metadata: HashMap<String, String>,
}

impl<T: Clone> Fixture<T> {
    /// Create a new fixture.
    pub fn new(name: &str, data: T) -> Self {
        Self {
            name: name.to_string(),
            data,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the fixture.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Get the fixture name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the fixture data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get metadata value.
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

/// A route fixture for testing routing scenarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteFixture {
    pub name: String,
    pub waypoints: Vec<WaypointFixture>,
    pub expected_distance_m: f64,
    pub expected_duration_s: f64,
    pub road_types: Vec<String>,
}

/// A waypoint in a route fixture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointFixture {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
}

impl WaypointFixture {
    /// Create a new waypoint fixture.
    pub fn new(lat: f64, lon: f64, name: &str) -> Self {
        Self {
            lat,
            lon,
            name: name.to_string(),
        }
    }
}

/// Route fixture builder.
pub struct RouteFixtureBuilder {
    name: String,
    waypoints: Vec<WaypointFixture>,
    expected_distance_m: f64,
    expected_duration_s: f64,
    road_types: Vec<String>,
}

impl RouteFixtureBuilder {
    /// Create a new route fixture builder.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waypoints: Vec::new(),
            expected_distance_m: 0.0,
            expected_duration_s: 0.0,
            road_types: Vec::new(),
        }
    }

    /// Add a waypoint.
    pub fn waypoint(mut self, lat: f64, lon: f64, name: &str) -> Self {
        self.waypoints.push(WaypointFixture::new(lat, lon, name));
        self
    }

    /// Set expected distance.
    pub fn expected_distance(mut self, meters: f64) -> Self {
        self.expected_distance_m = meters;
        self
    }

    /// Set expected duration.
    pub fn expected_duration(mut self, seconds: f64) -> Self {
        self.expected_duration_s = seconds;
        self
    }

    /// Add a road type.
    pub fn road_type(mut self, road_type: &str) -> Self {
        self.road_types.push(road_type.to_string());
        self
    }

    /// Build the route fixture.
    pub fn build(self) -> RouteFixture {
        RouteFixture {
            name: self.name,
            waypoints: self.waypoints,
            expected_distance_m: self.expected_distance_m,
            expected_duration_s: self.expected_duration_s,
            road_types: self.road_types,
        }
    }
}

/// Scenario fixture — a complete test scenario with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFixture {
    pub name: String,
    pub description: String,
    pub conditions: ScenarioConditions,
    pub expected_outcomes: Vec<String>,
}

/// Conditions for a test scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConditions {
    pub weather: String,
    pub time_of_day: String,
    pub traffic_level: String,
    pub connectivity: String,
    pub gnss_quality: String,
}

impl Default for ScenarioConditions {
    fn default() -> Self {
        Self {
            weather: "clear".to_string(),
            time_of_day: "daytime".to_string(),
            traffic_level: "normal".to_string(),
            connectivity: "online".to_string(),
            gnss_quality: "good".to_string(),
        }
    }
}

/// Pre-built fixture library with common test scenarios.
pub struct FixtureLibrary {
    routes: HashMap<String, RouteFixture>,
    scenarios: HashMap<String, ScenarioFixture>,
}

impl FixtureLibrary {
    /// Create a new fixture library with standard fixtures.
    pub fn new() -> Self {
        let mut lib = Self {
            routes: HashMap::new(),
            scenarios: HashMap::new(),
        };
        lib.load_standard_routes();
        lib.load_standard_scenarios();
        lib
    }

    fn load_standard_routes(&mut self) {
        // Tel Aviv to Jerusalem
        self.routes.insert(
            "tlv_to_jlm".to_string(),
            RouteFixtureBuilder::new("Tel Aviv to Jerusalem")
                .waypoint(32.0853, 34.7818, "Tel Aviv")
                .waypoint(31.7683, 35.2137, "Jerusalem")
                .expected_distance(63000.0)
                .expected_duration(3600.0)
                .road_type("highway")
                .build(),
        );

        // Short urban route
        self.routes.insert(
            "urban_short".to_string(),
            RouteFixtureBuilder::new("Urban Short Route")
                .waypoint(32.0800, 34.7800, "Start")
                .waypoint(32.0850, 34.7850, "End")
                .expected_distance(800.0)
                .expected_duration(180.0)
                .road_type("urban")
                .build(),
        );

        // Highway route
        self.routes.insert(
            "highway_straight".to_string(),
            RouteFixtureBuilder::new("Highway Straight")
                .waypoint(32.0, 34.5, "Entry")
                .waypoint(32.5, 34.5, "Exit")
                .expected_distance(55000.0)
                .expected_duration(1800.0)
                .road_type("highway")
                .build(),
        );
    }

    fn load_standard_scenarios(&mut self) {
        self.scenarios.insert(
            "normal_nav".to_string(),
            ScenarioFixture {
                name: "Normal Navigation".to_string(),
                description: "Standard daytime navigation with good conditions".to_string(),
                conditions: ScenarioConditions::default(),
                expected_outcomes: vec![
                    "Route calculated successfully".to_string(),
                    "Turn-by-turn instructions generated".to_string(),
                    "ETA within 10% accuracy".to_string(),
                ],
            },
        );

        self.scenarios.insert(
            "offline_nav".to_string(),
            ScenarioFixture {
                name: "Offline Navigation".to_string(),
                description: "Navigation without network connectivity".to_string(),
                conditions: ScenarioConditions {
                    connectivity: "offline".to_string(),
                    ..Default::default()
                },
                expected_outcomes: vec![
                    "Cached map data used".to_string(),
                    "Route calculated from offline data".to_string(),
                    "No network requests made".to_string(),
                ],
            },
        );

        self.scenarios.insert(
            "poor_gnss".to_string(),
            ScenarioFixture {
                name: "Poor GNSS Conditions".to_string(),
                description: "Navigation with degraded satellite reception".to_string(),
                conditions: ScenarioConditions {
                    gnss_quality: "poor".to_string(),
                    weather: "heavy_rain".to_string(),
                    ..Default::default()
                },
                expected_outcomes: vec![
                    "Fallback to dead reckoning".to_string(),
                    "Accuracy warning displayed".to_string(),
                    "Sensor fusion compensates".to_string(),
                ],
            },
        );
    }

    /// Get a route fixture by name.
    pub fn route(&self, name: &str) -> Option<&RouteFixture> {
        self.routes.get(name)
    }

    /// Get a scenario fixture by name.
    pub fn scenario(&self, name: &str) -> Option<&ScenarioFixture> {
        self.scenarios.get(name)
    }

    /// List available route fixture names.
    pub fn route_names(&self) -> Vec<String> {
        self.routes.keys().cloned().collect()
    }

    /// List available scenario fixture names.
    pub fn scenario_names(&self) -> Vec<String> {
        self.scenarios.keys().cloned().collect()
    }
}

impl Default for FixtureLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_creation() {
        let fixture = Fixture::new("test_data", vec![1, 2, 3]).with_metadata("source", "unit_test");
        assert_eq!(fixture.name(), "test_data");
        assert_eq!(fixture.data(), &vec![1, 2, 3]);
        assert_eq!(fixture.metadata("source"), Some("unit_test"));
    }

    #[test]
    fn test_route_fixture_builder() {
        let route = RouteFixtureBuilder::new("test_route")
            .waypoint(32.0, 34.0, "A")
            .waypoint(33.0, 35.0, "B")
            .expected_distance(100000.0)
            .expected_duration(3600.0)
            .road_type("highway")
            .road_type("urban")
            .build();

        assert_eq!(route.name, "test_route");
        assert_eq!(route.waypoints.len(), 2);
        assert_eq!(route.expected_distance_m, 100000.0);
        assert_eq!(route.road_types.len(), 2);
    }

    #[test]
    fn test_fixture_library_routes() {
        let lib = FixtureLibrary::new();

        let route = lib.route("tlv_to_jlm").unwrap();
        assert_eq!(route.waypoints.len(), 2);
        assert!(route.expected_distance_m > 0.0);

        assert!(lib.route("urban_short").is_some());
        assert!(lib.route("highway_straight").is_some());
        assert!(lib.route("nonexistent").is_none());
    }

    #[test]
    fn test_fixture_library_scenarios() {
        let lib = FixtureLibrary::new();

        let scenario = lib.scenario("normal_nav").unwrap();
        assert_eq!(scenario.conditions.connectivity, "online");
        assert!(!scenario.expected_outcomes.is_empty());

        let offline = lib.scenario("offline_nav").unwrap();
        assert_eq!(offline.conditions.connectivity, "offline");

        let poor = lib.scenario("poor_gnss").unwrap();
        assert_eq!(poor.conditions.gnss_quality, "poor");
    }

    #[test]
    fn test_scenario_conditions_default() {
        let cond = ScenarioConditions::default();
        assert_eq!(cond.weather, "clear");
        assert_eq!(cond.connectivity, "online");
        assert_eq!(cond.gnss_quality, "good");
    }

    #[test]
    fn test_fixture_library_names() {
        let lib = FixtureLibrary::new();
        assert_eq!(lib.route_names().len(), 3);
        assert_eq!(lib.scenario_names().len(), 3);
    }
}
