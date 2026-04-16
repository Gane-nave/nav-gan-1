/// Range estimator: energy model, terrain, climate, traffic
/// Phase 870

#[derive(Debug, Clone)]
pub struct RangeEstimator {
    pub energy_ok: bool,
    pub terrain_ok: bool,
    pub climate_ok: bool,
    pub traffic_ok: bool,
    pub accuracy_ok: bool,
}

impl Default for RangeEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeEstimator {
    pub fn new() -> Self {
        Self {
            energy_ok: true,
            terrain_ok: true,
            climate_ok: true,
            traffic_ok: true,
            accuracy_ok: true,
        }
    }

    pub fn modeling_ok(&self) -> bool {
        self.energy_ok && self.terrain_ok && self.climate_ok
    }

    pub fn prediction_ok(&self) -> bool {
        self.traffic_ok && self.accuracy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.modeling_ok() && self.prediction_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.energy_ok || !self.accuracy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.energy_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modeling() {
        let c = RangeEstimator::new();
        assert!(c.modeling_ok());
    }

    #[test]
    fn test_prediction() {
        let c = RangeEstimator::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RangeEstimator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = RangeEstimator::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_energy() {
        let mut c = RangeEstimator::new();
        c.energy_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = RangeEstimator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
