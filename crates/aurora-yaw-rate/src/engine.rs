/// Yaw rate sensor: rotation rate, stability control input, drift detection
/// Phase 339

#[derive(Debug, Clone)]
pub struct YawRate {
    pub rate_deg_s: f64,
    pub max_rate_deg_s: f64,
    pub bias_deg_s: f64,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for YawRate {
    fn default() -> Self {
        Self::new()
    }
}

impl YawRate {
    pub fn new() -> Self {
        Self {
            rate_deg_s: 0.0,
            max_rate_deg_s: 100.0,
            bias_deg_s: 0.01,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn stable(&self) -> bool {
        self.rate_deg_s.abs() < 5.0
    }

    pub fn spinning(&self) -> bool {
        self.rate_deg_s.abs() > self.max_rate_deg_s * 0.8
    }

    pub fn bias_ok(&self) -> bool {
        self.bias_deg_s.abs() < 0.5
    }

    pub fn ready(&self) -> bool {
        self.sensor_ok && self.calibrated && self.bias_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if !self.calibrated {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let y = YawRate::new();
        assert!(y.stable());
    }

    #[test]
    fn test_not_spinning() {
        let y = YawRate::new();
        assert!(!y.spinning());
    }

    #[test]
    fn test_bias_ok() {
        let y = YawRate::new();
        assert!(y.bias_ok());
    }

    #[test]
    fn test_ready() {
        let y = YawRate::new();
        assert!(y.ready());
    }

    #[test]
    fn test_spin() {
        let mut y = YawRate::new();
        y.rate_deg_s = 90.0;
        assert!(y.spinning());
    }

    #[test]
    fn test_health() {
        let y = YawRate::new();
        assert!((y.health_score() - 100.0).abs() < 0.1);
    }
}
