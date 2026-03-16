//! Traffic generation — synthetic traffic pattern generation for simulation and load testing.

use std::collections::HashMap;

/// Traffic density level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrafficDensity {
    /// No traffic.
    Empty,
    /// Light traffic — free flow.
    Light,
    /// Moderate traffic — some slowdowns.
    Moderate,
    /// Heavy traffic — significant delays.
    Heavy,
    /// Gridlock — near standstill.
    Gridlock,
}

/// A synthetic vehicle in the simulation.
#[derive(Debug, Clone)]
pub struct SimVehicle {
    /// Unique vehicle ID.
    pub id: u64,
    /// Current position (lat, lon).
    pub position: (f64, f64),
    /// Current speed (m/s).
    pub speed_mps: f64,
    /// Heading (degrees).
    pub heading_deg: f64,
    /// Route segment index.
    pub segment_idx: usize,
}

/// Traffic generation configuration.
#[derive(Debug, Clone)]
pub struct TrafficGenConfig {
    /// Vehicles per kilometre at each density level.
    pub density_rates: HashMap<String, f64>,
    /// Speed variance factor (0.0 = uniform, 1.0 = high variance).
    pub speed_variance: f64,
    /// Mean speed (m/s) for free-flow conditions.
    pub mean_free_flow_speed: f64,
    /// Random seed for deterministic generation.
    pub seed: u64,
}

impl Default for TrafficGenConfig {
    fn default() -> Self {
        let mut density_rates = HashMap::new();
        density_rates.insert("empty".to_string(), 0.0);
        density_rates.insert("light".to_string(), 5.0);
        density_rates.insert("moderate".to_string(), 15.0);
        density_rates.insert("heavy".to_string(), 30.0);
        density_rates.insert("gridlock".to_string(), 60.0);
        Self {
            density_rates,
            speed_variance: 0.2,
            mean_free_flow_speed: 27.78, // ~100 km/h
            seed: 42,
        }
    }
}

/// Traffic generator — creates synthetic traffic for simulation.
pub struct TrafficGenerator {
    config: TrafficGenConfig,
    vehicles: Vec<SimVehicle>,
    next_id: u64,
    rng_state: u64,
}

impl TrafficGenerator {
    /// Create a new traffic generator.
    pub fn new(config: TrafficGenConfig) -> Self {
        let seed = config.seed;
        Self {
            config,
            vehicles: Vec::new(),
            next_id: 1,
            rng_state: seed,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(TrafficGenConfig::default())
    }

    /// Simple deterministic PRNG (xorshift64).
    fn next_random(&mut self) -> f64 {
        if self.rng_state == 0 {
            self.rng_state = 1;
        }
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        (self.rng_state as f64) / (u64::MAX as f64)
    }

    /// Generate vehicles for a road segment.
    pub fn generate_segment(
        &mut self,
        start: (f64, f64),
        end: (f64, f64),
        density: TrafficDensity,
        length_km: f64,
    ) -> Vec<u64> {
        let density_key = match density {
            TrafficDensity::Empty => "empty",
            TrafficDensity::Light => "light",
            TrafficDensity::Moderate => "moderate",
            TrafficDensity::Heavy => "heavy",
            TrafficDensity::Gridlock => "gridlock",
        };

        let vehicles_per_km = self
            .config
            .density_rates
            .get(density_key)
            .copied()
            .unwrap_or(0.0);
        let count = (vehicles_per_km * length_km).round() as usize;

        let mut ids = Vec::new();
        for i in 0..count {
            let t = if count > 1 {
                i as f64 / (count - 1) as f64
            } else {
                0.5
            };

            let lat = start.0 + (end.0 - start.0) * t;
            let lon = start.1 + (end.1 - start.1) * t;

            let speed_factor = 1.0 - self.next_random() * self.config.speed_variance;
            let density_slowdown = match density {
                TrafficDensity::Empty => 1.0,
                TrafficDensity::Light => 0.9,
                TrafficDensity::Moderate => 0.65,
                TrafficDensity::Heavy => 0.35,
                TrafficDensity::Gridlock => 0.05,
            };
            let speed = self.config.mean_free_flow_speed * speed_factor * density_slowdown;

            let dlat = end.0 - start.0;
            let dlon = end.1 - start.1;
            let heading = dlon.atan2(dlat).to_degrees().rem_euclid(360.0);

            let id = self.next_id;
            self.next_id += 1;
            self.vehicles.push(SimVehicle {
                id,
                position: (lat, lon),
                speed_mps: speed.max(0.0),
                heading_deg: heading,
                segment_idx: 0,
            });
            ids.push(id);
        }
        ids
    }

    /// Get total vehicle count.
    pub fn vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    /// Get a vehicle by ID.
    pub fn get_vehicle(&self, id: u64) -> Option<&SimVehicle> {
        self.vehicles.iter().find(|v| v.id == id)
    }

    /// Get average speed of all vehicles.
    pub fn average_speed(&self) -> f64 {
        if self.vehicles.is_empty() {
            return 0.0;
        }
        let total: f64 = self.vehicles.iter().map(|v| v.speed_mps).sum();
        total / self.vehicles.len() as f64
    }

    /// Clear all vehicles.
    pub fn clear(&mut self) {
        self.vehicles.clear();
        self.next_id = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_density_no_vehicles() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Empty, 1.0);
        assert_eq!(ids.len(), 0);
        assert_eq!(gen.vehicle_count(), 0);
    }

    #[test]
    fn test_light_density() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        assert_eq!(ids.len(), 5); // 5 per km * 1 km
        assert_eq!(gen.vehicle_count(), 5);
    }

    #[test]
    fn test_heavy_density() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Heavy, 1.0);
        assert_eq!(ids.len(), 30); // 30 per km * 1 km
    }

    #[test]
    fn test_gridlock_density() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Gridlock, 1.0);
        assert_eq!(ids.len(), 60);
    }

    #[test]
    fn test_gridlock_slow_speed() {
        let mut gen = TrafficGenerator::with_defaults();
        gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Gridlock, 1.0);
        let avg = gen.average_speed();
        // Gridlock = 5% of free flow (27.78) ≈ 1.39 m/s, with variance
        assert!(
            avg < 3.0,
            "Gridlock average speed ({avg}) should be < 3.0 m/s"
        );
    }

    #[test]
    fn test_light_fast_speed() {
        let mut gen = TrafficGenerator::with_defaults();
        gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        let avg = gen.average_speed();
        // Light = 90% of free flow (27.78) ≈ 25 m/s, with variance
        assert!(
            avg > 15.0,
            "Light traffic average speed ({avg}) should be > 15.0 m/s"
        );
    }

    #[test]
    fn test_vehicle_positions_interpolated() {
        let mut gen = TrafficGenerator::with_defaults();
        gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        for v in &gen.vehicles {
            assert!(v.position.0 >= 32.0 && v.position.0 <= 32.01);
            assert!(v.position.1 >= 34.0 && v.position.1 <= 34.01);
        }
    }

    #[test]
    fn test_sequential_vehicle_ids() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids1 = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        let ids2 = gen.generate_segment((32.01, 34.01), (32.02, 34.02), TrafficDensity::Light, 1.0);
        assert_eq!(*ids1.last().unwrap() + 1, ids2[0]);
    }

    #[test]
    fn test_clear_resets() {
        let mut gen = TrafficGenerator::with_defaults();
        gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        assert!(gen.vehicle_count() > 0);
        gen.clear();
        assert_eq!(gen.vehicle_count(), 0);
    }

    #[test]
    fn test_get_vehicle() {
        let mut gen = TrafficGenerator::with_defaults();
        let ids = gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        assert!(gen.get_vehicle(ids[0]).is_some());
        assert!(gen.get_vehicle(9999).is_none());
    }

    #[test]
    fn test_average_speed_empty() {
        let gen = TrafficGenerator::with_defaults();
        assert_eq!(gen.average_speed(), 0.0);
    }

    #[test]
    fn test_density_ordering() {
        assert!(TrafficDensity::Empty < TrafficDensity::Light);
        assert!(TrafficDensity::Light < TrafficDensity::Moderate);
        assert!(TrafficDensity::Moderate < TrafficDensity::Heavy);
        assert!(TrafficDensity::Heavy < TrafficDensity::Gridlock);
    }

    #[test]
    fn test_zero_seed_not_stuck() {
        // Regression: xorshift64 absorbing state when seed == 0
        let mut gen = TrafficGenerator::new(TrafficGenConfig {
            seed: 0,
            ..Default::default()
        });
        gen.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Light, 1.0);
        // With seed=0 bug, all vehicles would have identical speeds
        let speeds: Vec<f64> = gen.vehicles.iter().map(|v| v.speed_mps).collect();
        assert!(
            speeds.len() > 1,
            "Need multiple vehicles to verify variance"
        );
        let all_same = speeds.windows(2).all(|w| (w[0] - w[1]).abs() < 1e-10);
        assert!(!all_same, "Zero seed should not produce identical speeds");
    }

    #[test]
    fn test_deterministic_generation() {
        // Two generators with the same seed should produce the same vehicles
        let mut gen1 = TrafficGenerator::new(TrafficGenConfig {
            seed: 123,
            ..Default::default()
        });
        let mut gen2 = TrafficGenerator::new(TrafficGenConfig {
            seed: 123,
            ..Default::default()
        });

        gen1.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Moderate, 1.0);
        gen2.generate_segment((32.0, 34.0), (32.01, 34.01), TrafficDensity::Moderate, 1.0);

        assert_eq!(gen1.vehicle_count(), gen2.vehicle_count());
        for (v1, v2) in gen1.vehicles.iter().zip(gen2.vehicles.iter()) {
            assert!((v1.speed_mps - v2.speed_mps).abs() < 1e-10);
            assert!((v1.position.0 - v2.position.0).abs() < 1e-10);
        }
    }
}
