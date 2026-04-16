/// Steering angle sensor: absolute position, rate of change, centering
/// Phase 338

#[derive(Debug, Clone)]
pub struct SteerAngle {
    pub angle_deg: f64,
    pub rate_deg_s: f64,
    pub centered: bool,
    pub calibrated: bool,
    pub sensor_ok: bool,
}

impl Default for SteerAngle {
    fn default() -> Self {
        Self::new()
    }
}

impl SteerAngle {
    pub fn new() -> Self {
        Self {
            angle_deg: 0.0,
            rate_deg_s: 0.0,
            centered: true,
            calibrated: true,
            sensor_ok: true,
        }
    }

    pub fn is_centered(&self) -> bool {
        self.angle_deg.abs() < 5.0
    }

    pub fn turning(&self) -> bool {
        self.rate_deg_s.abs() > 2.0
    }

    pub fn full_lock(&self) -> bool {
        self.angle_deg.abs() > 500.0
    }

    pub fn ready(&self) -> bool {
        self.calibrated && self.sensor_ok
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
    fn test_centered() {
        let s = SteerAngle::new();
        assert!(s.is_centered());
    }

    #[test]
    fn test_not_turning() {
        let s = SteerAngle::new();
        assert!(!s.turning());
    }

    #[test]
    fn test_not_full() {
        let s = SteerAngle::new();
        assert!(!s.full_lock());
    }

    #[test]
    fn test_ready() {
        let s = SteerAngle::new();
        assert!(s.ready());
    }

    #[test]
    fn test_turning() {
        let mut s = SteerAngle::new();
        s.rate_deg_s = 10.0;
        assert!(s.turning());
    }

    #[test]
    fn test_health() {
        let s = SteerAngle::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
