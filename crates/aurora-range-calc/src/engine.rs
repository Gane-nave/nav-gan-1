/// Range calculator: remaining range estimation, terrain factor, climate impact
/// Phase 288

#[derive(Debug, Clone)]
pub struct RangeCalculator {
    pub estimated_range_km: f64,
    pub energy_remaining_kwh: f64,
    pub consumption_kwh_per_km: f64,
    pub terrain_factor: f64,
    pub climate_factor: f64,
}

impl Default for RangeCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeCalculator {
    pub fn new() -> Self {
        Self {
            estimated_range_km: 300.0,
            energy_remaining_kwh: 50.0,
            consumption_kwh_per_km: 0.17,
            terrain_factor: 1.0,
            climate_factor: 1.0,
        }
    }

    pub fn adjusted_range(&self) -> f64 {
        if self.consumption_kwh_per_km <= 0.0 {
            return 0.0;
        }
        self.energy_remaining_kwh
            / (self.consumption_kwh_per_km * self.terrain_factor * self.climate_factor)
    }

    pub fn range_ok(&self) -> bool {
        self.adjusted_range() > 30.0
    }

    pub fn low_range(&self) -> bool {
        self.adjusted_range() < 50.0
    }

    pub fn critical_range(&self) -> bool {
        self.adjusted_range() < 15.0
    }

    pub fn health_score(&self) -> f64 {
        if self.critical_range() {
            return 10.0;
        }
        if self.low_range() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let r = RangeCalculator::new();
        assert!(r.adjusted_range() > 200.0);
    }

    #[test]
    fn test_range_ok() {
        let r = RangeCalculator::new();
        assert!(r.range_ok());
    }

    #[test]
    fn test_not_low() {
        let r = RangeCalculator::new();
        assert!(!r.low_range());
    }

    #[test]
    fn test_not_critical() {
        let r = RangeCalculator::new();
        assert!(!r.critical_range());
    }

    #[test]
    fn test_low() {
        let mut r = RangeCalculator::new();
        r.energy_remaining_kwh = 5.0;
        assert!(r.low_range());
    }

    #[test]
    fn test_health() {
        let r = RangeCalculator::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
