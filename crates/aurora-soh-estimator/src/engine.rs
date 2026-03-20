/// SOH estimator: capacity fade, resistance growth, cycle
/// Phase 869

#[derive(Debug, Clone)]
pub struct SohEstimator {
    pub capacity_ok: bool,
    pub resistance_ok: bool,
    pub cycle_ok: bool,
    pub model_ok: bool,
    pub accuracy_ok: bool,
}

impl Default for SohEstimator {
    fn default() -> Self {
        Self::new()
    }
}

impl SohEstimator {
    pub fn new() -> Self {
        Self {
            capacity_ok: true,
            resistance_ok: true,
            cycle_ok: true,
            model_ok: true,
            accuracy_ok: true,
        }
    }

    pub fn degradation_ok(&self) -> bool {
        self.capacity_ok && self.resistance_ok && self.cycle_ok
    }

    pub fn prediction_ok(&self) -> bool {
        self.model_ok && self.accuracy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.degradation_ok() && self.prediction_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.model_ok || !self.accuracy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.model_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degradation() {
        let c = SohEstimator::new();
        assert!(c.degradation_ok());
    }

    #[test]
    fn test_prediction() {
        let c = SohEstimator::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SohEstimator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = SohEstimator::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_model() {
        let mut c = SohEstimator::new();
        c.model_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = SohEstimator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
