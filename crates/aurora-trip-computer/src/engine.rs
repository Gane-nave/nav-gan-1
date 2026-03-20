/// Trip computer: distance, time, average speed, fuel consumed
/// Phase 286

#[derive(Debug, Clone)]
pub struct TripComputer {
    pub distance_km: f64,
    pub time_hours: f64,
    pub fuel_used_l: f64,
    pub max_speed_kmh: f64,
    pub idle_time_min: f64,
}

impl Default for TripComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl TripComputer {
    pub fn new() -> Self {
        Self {
            distance_km: 0.0,
            time_hours: 0.0,
            fuel_used_l: 0.0,
            max_speed_kmh: 0.0,
            idle_time_min: 0.0,
        }
    }

    pub fn avg_speed_kmh(&self) -> f64 {
        if self.time_hours <= 0.0 {
            return 0.0;
        }
        self.distance_km / self.time_hours
    }

    pub fn fuel_economy_l100km(&self) -> f64 {
        if self.distance_km <= 0.0 {
            return 0.0;
        }
        self.fuel_used_l / self.distance_km * 100.0
    }

    pub fn has_data(&self) -> bool {
        self.distance_km > 0.0 || self.time_hours > 0.0
    }

    pub fn idle_pct(&self) -> f64 {
        if self.time_hours <= 0.0 {
            return 0.0;
        }
        let total_min = self.time_hours * 60.0;
        (self.idle_time_min / total_min * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_data() {
        let t = TripComputer::new();
        assert!(!t.has_data());
    }

    #[test]
    fn test_zero_speed() {
        let t = TripComputer::new();
        assert!(t.avg_speed_kmh() < 0.1);
    }

    #[test]
    fn test_zero_economy() {
        let t = TripComputer::new();
        assert!(t.fuel_economy_l100km() < 0.1);
    }

    #[test]
    fn test_zero_idle() {
        let t = TripComputer::new();
        assert!(t.idle_pct() < 0.1);
    }

    #[test]
    fn test_with_data() {
        let mut t = TripComputer::new();
        t.distance_km = 100.0;
        t.time_hours = 1.5;
        assert!(t.avg_speed_kmh() > 60.0);
    }

    #[test]
    fn test_health() {
        let t = TripComputer::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
