/// Traffic prediction: historical, realtime, model, predict, evaluate
/// Phase 1093

#[derive(Debug, Clone)]
pub struct TrafficPred {
    pub historical_ok: bool,
    pub realtime_ok: bool,
    pub model_ok: bool,
    pub predict_ok: bool,
    pub evaluate_ok: bool,
}

impl Default for TrafficPred {
    fn default() -> Self {
        Self::new()
    }
}

impl TrafficPred {
    pub fn new() -> Self {
        Self {
            historical_ok: true,
            realtime_ok: true,
            model_ok: true,
            predict_ok: true,
            evaluate_ok: true,
        }
    }

    pub fn data_ok(&self) -> bool {
        self.historical_ok && self.realtime_ok && self.model_ok
    }

    pub fn output_ok(&self) -> bool {
        self.predict_ok && self.evaluate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.data_ok() && self.output_ok()
    }

    pub fn needs_retrain(&self) -> bool {
        !self.model_ok || !self.historical_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.historical_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let c = TrafficPred::new();
        assert!(c.data_ok());
    }

    #[test]
    fn test_output() {
        let c = TrafficPred::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrafficPred::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retrain() {
        let c = TrafficPred::new();
        assert!(!c.needs_retrain());
    }

    #[test]
    fn test_model() {
        let mut c = TrafficPred::new();
        c.model_ok = false;
        assert!(c.needs_retrain());
    }

    #[test]
    fn test_health() {
        let c = TrafficPred::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
