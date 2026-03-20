/// Predictive maintenance: sensor, model, predict, schedule, report
/// Phase 1097

#[derive(Debug, Clone)]
pub struct MaintPred {
    pub sensor_ok: bool,
    pub model_ok: bool,
    pub predict_ok: bool,
    pub schedule_ok: bool,
    pub report_ok: bool,
}

impl Default for MaintPred {
    fn default() -> Self {
        Self::new()
    }
}

impl MaintPred {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            model_ok: true,
            predict_ok: true,
            schedule_ok: true,
            report_ok: true,
        }
    }

    pub fn prediction_ok(&self) -> bool {
        self.sensor_ok && self.model_ok && self.predict_ok
    }

    pub fn planning_ok(&self) -> bool {
        self.schedule_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.prediction_ok() && self.planning_ok()
    }

    pub fn needs_retrain(&self) -> bool {
        !self.model_ok || !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction() {
        let c = MaintPred::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_planning() {
        let c = MaintPred::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MaintPred::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retrain() {
        let c = MaintPred::new();
        assert!(!c.needs_retrain());
    }

    #[test]
    fn test_model() {
        let mut c = MaintPred::new();
        c.model_ok = false;
        assert!(c.needs_retrain());
    }

    #[test]
    fn test_health() {
        let c = MaintPred::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
