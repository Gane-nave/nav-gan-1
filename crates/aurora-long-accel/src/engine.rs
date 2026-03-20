/// Longitudinal acceleration: braking/acceleration g-force, pitch detection
/// Phase 341

#[derive(Debug, Clone)]
pub struct LongAccel {
    pub accel_g: f64,
    pub max_brake_g: f64,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for LongAccel {
    fn default() -> Self {
        Self::new()
    }
}

impl LongAccel {
    pub fn new() -> Self {
        Self {
            accel_g: 0.0,
            max_brake_g: 1.0,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn accelerating(&self) -> bool {
        self.accel_g > 0.05
    }

    pub fn braking(&self) -> bool {
        self.accel_g < -0.05
    }

    pub fn hard_braking(&self) -> bool {
        self.accel_g < -0.6
    }

    pub fn emergency_stop(&self) -> bool {
        self.accel_g < -self.max_brake_g * 0.9
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
    fn test_not_accel() {
        let l = LongAccel::new();
        assert!(!l.accelerating());
    }

    #[test]
    fn test_not_braking() {
        let l = LongAccel::new();
        assert!(!l.braking());
    }

    #[test]
    fn test_not_hard() {
        let l = LongAccel::new();
        assert!(!l.hard_braking());
    }

    #[test]
    fn test_no_emergency() {
        let l = LongAccel::new();
        assert!(!l.emergency_stop());
    }

    #[test]
    fn test_braking() {
        let mut l = LongAccel::new();
        l.accel_g = -0.7;
        assert!(l.hard_braking());
    }

    #[test]
    fn test_health() {
        let l = LongAccel::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
