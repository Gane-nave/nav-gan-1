//! Mock objects — configurable mock implementations for testing navigation subsystems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mock response configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockResponse<T> {
    /// Return a fixed value.
    Value(T),
    /// Return an error.
    Error(String),
    /// Return values in sequence, cycling when exhausted.
    Sequence(Vec<T>),
}

/// A mock GNSS provider for testing.
pub struct MockGnssProvider {
    positions: Vec<MockPosition>,
    current_index: parking_lot::RwLock<usize>,
    call_count: parking_lot::RwLock<u64>,
    should_fail: parking_lot::RwLock<bool>,
    failure_message: parking_lot::RwLock<String>,
}

/// A mock position fix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy_m: f64,
    pub timestamp_ms: u64,
}

impl MockPosition {
    /// Create a new mock position.
    pub fn new(lat: f64, lon: f64, alt: f64, accuracy: f64) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
            altitude: alt,
            accuracy_m: accuracy,
            timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

impl MockGnssProvider {
    /// Create a new mock GNSS provider with a set of positions to cycle through.
    pub fn new(positions: Vec<MockPosition>) -> Self {
        assert!(!positions.is_empty(), "Must provide at least one position");
        Self {
            positions,
            current_index: parking_lot::RwLock::new(0),
            call_count: parking_lot::RwLock::new(0),
            should_fail: parking_lot::RwLock::new(false),
            failure_message: parking_lot::RwLock::new("Mock GNSS failure".to_string()),
        }
    }

    /// Create with a single fixed position.
    pub fn fixed(lat: f64, lon: f64) -> Self {
        Self::new(vec![MockPosition::new(lat, lon, 0.0, 1.0)])
    }

    /// Set the provider to fail on next call.
    pub fn set_should_fail(&self, fail: bool, message: &str) {
        *self.should_fail.write() = fail;
        *self.failure_message.write() = message.to_string();
    }

    /// Get the next position fix.
    pub fn next_fix(&self) -> Result<MockPosition, String> {
        *self.call_count.write() += 1;

        if *self.should_fail.read() {
            return Err(self.failure_message.read().clone());
        }

        let mut index = self.current_index.write();
        let pos = self.positions[*index].clone();
        *index = (*index + 1) % self.positions.len();
        Ok(pos)
    }

    /// Get the total number of calls made.
    pub fn call_count(&self) -> u64 {
        *self.call_count.read()
    }

    /// Reset the provider state.
    pub fn reset(&self) {
        *self.current_index.write() = 0;
        *self.call_count.write() = 0;
        *self.should_fail.write() = false;
    }
}

/// A mock sensor provider for testing.
pub struct MockSensorProvider {
    readings: HashMap<String, Vec<f64>>,
    indices: parking_lot::RwLock<HashMap<String, usize>>,
    call_counts: parking_lot::RwLock<HashMap<String, u64>>,
}

impl MockSensorProvider {
    /// Create a new mock sensor provider.
    pub fn new() -> Self {
        Self {
            readings: HashMap::new(),
            indices: parking_lot::RwLock::new(HashMap::new()),
            call_counts: parking_lot::RwLock::new(HashMap::new()),
        }
    }

    /// Add readings for a sensor type.
    pub fn add_sensor(&mut self, sensor_type: &str, readings: Vec<f64>) {
        self.readings.insert(sensor_type.to_string(), readings);
    }

    /// Get the next reading for a sensor type.
    pub fn next_reading(&self, sensor_type: &str) -> Option<f64> {
        let readings = self.readings.get(sensor_type)?;
        let mut indices = self.indices.write();
        let mut counts = self.call_counts.write();

        let index = indices.entry(sensor_type.to_string()).or_insert(0);
        *counts.entry(sensor_type.to_string()).or_insert(0) += 1;

        let value = readings[*index % readings.len()];
        *index += 1;
        Some(value)
    }

    /// Get call count for a sensor type.
    pub fn call_count(&self, sensor_type: &str) -> u64 {
        self.call_counts
            .read()
            .get(sensor_type)
            .copied()
            .unwrap_or(0)
    }

    /// Reset all sensor state.
    pub fn reset(&self) {
        self.indices.write().clear();
        self.call_counts.write().clear();
    }
}

impl Default for MockSensorProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// A mock network client for testing API interactions.
pub struct MockNetworkClient {
    responses: parking_lot::RwLock<HashMap<String, Vec<MockHttpResponse>>>,
    request_log: parking_lot::RwLock<Vec<MockHttpRequest>>,
}

/// A recorded HTTP request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockHttpRequest {
    pub method: String,
    pub url: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

/// A mock HTTP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockHttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl MockNetworkClient {
    /// Create a new mock network client.
    pub fn new() -> Self {
        Self {
            responses: parking_lot::RwLock::new(HashMap::new()),
            request_log: parking_lot::RwLock::new(Vec::new()),
        }
    }

    /// Register a mock response for a URL pattern.
    pub fn on_request(&self, url: &str, response: MockHttpResponse) {
        let mut responses = self.responses.write();
        responses.entry(url.to_string()).or_default().push(response);
    }

    /// Simulate an HTTP request.
    pub fn request(&self, req: MockHttpRequest) -> MockHttpResponse {
        self.request_log.write().push(req.clone());

        let mut responses = self.responses.write();
        if let Some(queue) = responses.get_mut(&req.url) {
            if !queue.is_empty() {
                return queue.remove(0);
            }
        }

        // Default 404 if no mock configured
        MockHttpResponse {
            status: 404,
            body: "Not found".to_string(),
            headers: HashMap::new(),
        }
    }

    /// Get all recorded requests.
    pub fn recorded_requests(&self) -> Vec<MockHttpRequest> {
        self.request_log.read().clone()
    }

    /// Get requests matching a URL.
    pub fn requests_to(&self, url: &str) -> Vec<MockHttpRequest> {
        self.request_log
            .read()
            .iter()
            .filter(|r| r.url == url)
            .cloned()
            .collect()
    }

    /// Total number of requests made.
    pub fn request_count(&self) -> usize {
        self.request_log.read().len()
    }

    /// Clear all mocks and recorded requests.
    pub fn reset(&self) {
        self.responses.write().clear();
        self.request_log.write().clear();
    }
}

impl Default for MockNetworkClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_gnss_fixed() {
        let provider = MockGnssProvider::fixed(32.0853, 34.7818);
        let fix = provider.next_fix().unwrap();
        assert!((fix.latitude - 32.0853).abs() < 0.0001);
        assert!((fix.longitude - 34.7818).abs() < 0.0001);
        assert_eq!(provider.call_count(), 1);
    }

    #[test]
    fn test_mock_gnss_sequence() {
        let positions = vec![
            MockPosition::new(32.0, 34.0, 0.0, 1.0),
            MockPosition::new(33.0, 35.0, 0.0, 1.0),
        ];
        let provider = MockGnssProvider::new(positions);

        let fix1 = provider.next_fix().unwrap();
        assert!((fix1.latitude - 32.0).abs() < 0.01);

        let fix2 = provider.next_fix().unwrap();
        assert!((fix2.latitude - 33.0).abs() < 0.01);

        // Cycles back
        let fix3 = provider.next_fix().unwrap();
        assert!((fix3.latitude - 32.0).abs() < 0.01);

        assert_eq!(provider.call_count(), 3);
    }

    #[test]
    fn test_mock_gnss_failure() {
        let provider = MockGnssProvider::fixed(0.0, 0.0);
        provider.set_should_fail(true, "Signal lost");
        let result = provider.next_fix();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Signal lost");
    }

    #[test]
    fn test_mock_gnss_reset() {
        let provider = MockGnssProvider::fixed(0.0, 0.0);
        provider.next_fix().unwrap();
        provider.next_fix().unwrap();
        assert_eq!(provider.call_count(), 2);
        provider.reset();
        assert_eq!(provider.call_count(), 0);
    }

    #[test]
    fn test_mock_sensor_provider() {
        let mut sensors = MockSensorProvider::new();
        sensors.add_sensor("imu_accel", vec![9.8, 9.7, 9.9]);
        sensors.add_sensor("gyro", vec![0.01, -0.02]);

        assert!((sensors.next_reading("imu_accel").unwrap() - 9.8).abs() < 0.01);
        assert!((sensors.next_reading("imu_accel").unwrap() - 9.7).abs() < 0.01);
        assert_eq!(sensors.call_count("imu_accel"), 2);
        assert_eq!(sensors.call_count("gyro"), 0);
    }

    #[test]
    fn test_mock_sensor_unknown_type() {
        let sensors = MockSensorProvider::new();
        assert!(sensors.next_reading("unknown").is_none());
    }

    #[test]
    fn test_mock_network_client() {
        let client = MockNetworkClient::new();
        client.on_request(
            "/api/route",
            MockHttpResponse {
                status: 200,
                body: r#"{"distance": 1500}"#.to_string(),
                headers: HashMap::new(),
            },
        );

        let resp = client.request(MockHttpRequest {
            method: "GET".to_string(),
            url: "/api/route".to_string(),
            body: None,
            headers: HashMap::new(),
        });

        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("1500"));
        assert_eq!(client.request_count(), 1);
    }

    #[test]
    fn test_mock_network_client_default_404() {
        let client = MockNetworkClient::new();
        let resp = client.request(MockHttpRequest {
            method: "GET".to_string(),
            url: "/unknown".to_string(),
            body: None,
            headers: HashMap::new(),
        });
        assert_eq!(resp.status, 404);
    }

    #[test]
    fn test_mock_network_recorded_requests() {
        let client = MockNetworkClient::new();
        client.request(MockHttpRequest {
            method: "POST".to_string(),
            url: "/api/telemetry".to_string(),
            body: Some("data".to_string()),
            headers: HashMap::new(),
        });
        client.request(MockHttpRequest {
            method: "GET".to_string(),
            url: "/api/status".to_string(),
            body: None,
            headers: HashMap::new(),
        });

        let telemetry_reqs = client.requests_to("/api/telemetry");
        assert_eq!(telemetry_reqs.len(), 1);
        assert_eq!(telemetry_reqs[0].method, "POST");
        assert_eq!(client.request_count(), 2);
    }
}
