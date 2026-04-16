/// Lateral acceleration: cornering force, g-force measurement, rollover
/// Phase 340

#[derive(Debug, Clone)]
pub struct LatAccel {
    pub accel_g: f64,
    pub max_g: f64,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for LatAccel {
    fn default() -> Self {
        Self::new()
    }
}

impl LatAccel {
    pub fn new() -> Self {
        Self {
            accel_g: 0.0,
            max_g: 1.2,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn cornering(&self) -> bool {
        self.accel_g.abs() > 0.1
    }

    pub fn hard_cornering(&self) -> bool {
        self.accel_g.abs() > 0.8
    }

    pub fn rollover_risk(&self) -> bool {
        self.accel_g.abs() > self.max_g * 0.9
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
    fn test_not_cornering() {
        let l = LatAccel::new();
        assert!(!l.cornering());
    }

    #[test]
    fn test_not_hard() {
        let l = LatAccel::new();
        assert!(!l.hard_cornering());
    }

    #[test]
    fn test_no_rollover() {
        let l = LatAccel::new();
        assert!(!l.rollover_risk());
    }

    #[test]
    fn test_ready() {
        let l = LatAccel::new();
        assert!(l.ready());
    }

    #[test]
    fn test_hard() {
        let mut l = LatAccel::new();
        l.accel_g = 0.9;
        assert!(l.hard_cornering());
    }

    #[test]
    fn test_health() {
        let l = LatAccel::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
