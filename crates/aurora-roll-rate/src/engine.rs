/// Roll rate sensor: body roll measurement, anti-roll control
/// Phase 342

#[derive(Debug, Clone)]
pub struct RollRate {
    pub rate_deg_s: f64,
    pub max_rate_deg_s: f64,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for RollRate {
    fn default() -> Self {
        Self::new()
    }
}

impl RollRate {
    pub fn new() -> Self {
        Self {
            rate_deg_s: 0.0,
            max_rate_deg_s: 30.0,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn stable(&self) -> bool {
        self.rate_deg_s.abs() < 3.0
    }

    pub fn excessive_roll(&self) -> bool {
        self.rate_deg_s.abs() > self.max_rate_deg_s * 0.7
    }

    pub fn rollover_risk(&self) -> bool {
        self.rate_deg_s.abs() > self.max_rate_deg_s * 0.9
    }

    pub fn ready(&self) -> bool {
        self.sensor_ok && self.calibrated
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
        let r = RollRate::new();
        assert!(r.stable());
    }

    #[test]
    fn test_no_excessive() {
        let r = RollRate::new();
        assert!(!r.excessive_roll());
    }

    #[test]
    fn test_no_rollover() {
        let r = RollRate::new();
        assert!(!r.rollover_risk());
    }

    #[test]
    fn test_ready() {
        let r = RollRate::new();
        assert!(r.ready());
    }

    #[test]
    fn test_excessive() {
        let mut r = RollRate::new();
        r.rate_deg_s = 25.0;
        assert!(r.excessive_roll());
    }

    #[test]
    fn test_health() {
        let r = RollRate::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
