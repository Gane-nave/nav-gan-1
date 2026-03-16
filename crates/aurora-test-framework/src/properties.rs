//! Property-based testing — generators, shrinking, and property validators.

use serde::{Deserialize, Serialize};

/// A simple pseudo-random number generator for deterministic property tests.
pub struct TestRng {
    state: u64,
}

impl TestRng {
    /// Create a new RNG with the given seed.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate the next pseudo-random u64.
    pub fn next_u64(&mut self) -> u64 {
        // xorshift64 — state must never be 0
        if self.state == 0 {
            self.state = 1;
        }
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// Generate a random f64 in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Generate a random f64 in [min, max).
    pub fn next_f64_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }

    /// Generate a random u64 in [min, max).
    pub fn next_u64_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        min + self.next_u64() % (max - min)
    }

    /// Generate a random boolean.
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() % 2 == 0
    }
}

/// A coordinate generator for testing navigation properties.
pub struct CoordinateGenerator {
    rng: TestRng,
}

impl CoordinateGenerator {
    /// Create a new coordinate generator with the given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: TestRng::new(seed),
        }
    }

    /// Generate a random valid latitude (-90 to 90).
    pub fn latitude(&mut self) -> f64 {
        self.rng.next_f64_range(-90.0, 90.0)
    }

    /// Generate a random valid longitude (-180 to 180).
    pub fn longitude(&mut self) -> f64 {
        self.rng.next_f64_range(-180.0, 180.0)
    }

    /// Generate a coordinate pair (lat, lon).
    pub fn coordinate(&mut self) -> (f64, f64) {
        (self.latitude(), self.longitude())
    }

    /// Generate N coordinate pairs.
    pub fn coordinates(&mut self, n: usize) -> Vec<(f64, f64)> {
        (0..n).map(|_| self.coordinate()).collect()
    }

    /// Generate a coordinate near a reference point (within radius_deg degrees).
    pub fn near(&mut self, lat: f64, lon: f64, radius_deg: f64) -> (f64, f64) {
        let dlat = self.rng.next_f64_range(-radius_deg, radius_deg);
        let dlon = self.rng.next_f64_range(-radius_deg, radius_deg);
        (
            (lat + dlat).clamp(-90.0, 90.0),
            (lon + dlon).clamp(-180.0, 180.0),
        )
    }
}

/// Result of a property test run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTestResult {
    pub property_name: String,
    pub iterations: u64,
    pub passed: u64,
    pub failed: u64,
    pub first_failure: Option<String>,
    pub seed: u64,
}

impl PropertyTestResult {
    /// Check if all iterations passed.
    pub fn all_passed(&self) -> bool {
        self.failed == 0
    }

    /// Get the failure rate.
    pub fn failure_rate(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            self.failed as f64 / self.iterations as f64
        }
    }
}

/// Property test runner — executes property-based tests with configurable iterations.
pub struct PropertyRunner {
    iterations: u64,
    seed: u64,
}

impl PropertyRunner {
    /// Create a new property runner.
    pub fn new(iterations: u64, seed: u64) -> Self {
        Self { iterations, seed }
    }

    /// Run a property test. The property function receives a TestRng and returns
    /// Ok(()) if the property holds, or Err(description) if it fails.
    pub fn run<F>(&self, name: &str, mut property: F) -> PropertyTestResult
    where
        F: FnMut(&mut TestRng) -> Result<(), String>,
    {
        let mut passed = 0u64;
        let mut failed = 0u64;
        let mut first_failure = None;

        for i in 0..self.iterations {
            let mut rng = TestRng::new(self.seed.wrapping_add(i));
            match property(&mut rng) {
                Ok(()) => passed += 1,
                Err(msg) => {
                    if first_failure.is_none() {
                        first_failure = Some(format!("Iteration {i}: {msg}"));
                    }
                    failed += 1;
                }
            }
        }

        PropertyTestResult {
            property_name: name.to_string(),
            iterations: self.iterations,
            passed,
            failed,
            first_failure,
            seed: self.seed,
        }
    }
}

impl Default for PropertyRunner {
    fn default() -> Self {
        Self::new(100, 42)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_deterministic() {
        let mut rng1 = TestRng::new(42);
        let mut rng2 = TestRng::new(42);
        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_rng_different_seeds() {
        let mut rng1 = TestRng::new(42);
        let mut rng2 = TestRng::new(43);
        // With overwhelming probability, different seeds produce different sequences
        let v1: Vec<u64> = (0..10).map(|_| rng1.next_u64()).collect();
        let v2: Vec<u64> = (0..10).map(|_| rng2.next_u64()).collect();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_rng_f64_range() {
        let mut rng = TestRng::new(42);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn test_coordinate_generator_valid_ranges() {
        let mut gen = CoordinateGenerator::new(42);
        for _ in 0..1000 {
            let (lat, lon) = gen.coordinate();
            assert!((-90.0..=90.0).contains(&lat), "Invalid lat: {lat}");
            assert!((-180.0..=180.0).contains(&lon), "Invalid lon: {lon}");
        }
    }

    #[test]
    fn test_coordinate_near() {
        let mut gen = CoordinateGenerator::new(42);
        let center_lat = 32.0;
        let center_lon = 34.0;
        let radius = 0.1;

        for _ in 0..100 {
            let (lat, lon) = gen.near(center_lat, center_lon, radius);
            assert!((lat - center_lat).abs() <= radius + f64::EPSILON);
            assert!((lon - center_lon).abs() <= radius + f64::EPSILON);
        }
    }

    #[test]
    fn test_property_runner_all_pass() {
        let runner = PropertyRunner::new(50, 42);
        let result = runner.run("always_true", |_rng| Ok(()));
        assert!(result.all_passed());
        assert_eq!(result.iterations, 50);
        assert_eq!(result.passed, 50);
        assert_eq!(result.failed, 0);
        assert!(result.first_failure.is_none());
    }

    #[test]
    fn test_property_runner_some_fail() {
        let runner = PropertyRunner::new(100, 42);
        let result = runner.run("sometimes_fails", |rng| {
            let v = rng.next_f64();
            if v < 0.1 {
                Err(format!("Value {v} too small"))
            } else {
                Ok(())
            }
        });
        assert!(!result.all_passed());
        assert!(result.failed > 0);
        assert!(result.first_failure.is_some());
    }

    #[test]
    fn test_property_runner_reproducible() {
        let runner = PropertyRunner::new(50, 42);
        let result1 = runner.run("test", |rng| {
            if rng.next_f64() < 0.05 {
                Err("fail".to_string())
            } else {
                Ok(())
            }
        });
        let result2 = runner.run("test", |rng| {
            if rng.next_f64() < 0.05 {
                Err("fail".to_string())
            } else {
                Ok(())
            }
        });
        assert_eq!(result1.passed, result2.passed);
        assert_eq!(result1.failed, result2.failed);
    }

    #[test]
    fn test_coordinates_batch() {
        let mut gen = CoordinateGenerator::new(42);
        let coords = gen.coordinates(5);
        assert_eq!(coords.len(), 5);
    }
}
