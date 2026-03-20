/// Wheel odometry: tick, speed, distance, slip, fuse
/// Phase 1109

#[derive(Debug, Clone)]
pub struct WheelOdom {
    pub tick_ok: bool,
    pub speed_ok: bool,
    pub distance_ok: bool,
    pub slip_ok: bool,
    pub fuse_ok: bool,
}

impl Default for WheelOdom {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelOdom {
    pub fn new() -> Self {
        Self {
            tick_ok: true,
            speed_ok: true,
            distance_ok: true,
            slip_ok: true,
            fuse_ok: true,
        }
    }

    pub fn measurement_ok(&self) -> bool {
        self.tick_ok && self.speed_ok && self.distance_ok
    }

    pub fn correction_ok(&self) -> bool {
        self.slip_ok && self.fuse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.measurement_ok() && self.correction_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.tick_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tick_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement() {
        let c = WheelOdom::new();
        assert!(c.measurement_ok());
    }

    #[test]
    fn test_correction() {
        let c = WheelOdom::new();
        assert!(c.correction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelOdom::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = WheelOdom::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_tick() {
        let mut c = WheelOdom::new();
        c.tick_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = WheelOdom::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
