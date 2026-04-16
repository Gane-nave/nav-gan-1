/// aurora-sim-sensor: sim sensor
/// Phase 2523

#[derive(Debug, Clone)]
pub struct SimSensor {
    pub generate_ok: bool,
    pub noise_ok: bool,
    pub drift_ok: bool,
    pub failure_ok: bool,
    pub calibrate_ok: bool,
}

impl Default for SimSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl SimSensor {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            noise_ok: true,
            drift_ok: true,
            failure_ok: true,
            calibrate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.noise_ok && self.drift_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.failure_ok && self.calibrate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.noise_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SimSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimSensor::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimSensor::default();
        assert!(c.all_ok());
    }
}
