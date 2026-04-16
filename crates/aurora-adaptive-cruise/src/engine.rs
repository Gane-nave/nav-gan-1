/// Adaptive cruise control: radar, speed, gap, stop-go
/// Phase 738

#[derive(Debug, Clone)]
pub struct AdaptiveCruise {
    pub radar_ok: bool,
    pub speed_ctrl_ok: bool,
    pub gap_ok: bool,
    pub stop_go_ok: bool,
    pub calibrated: bool,
}

impl Default for AdaptiveCruise {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveCruise {
    pub fn new() -> Self {
        Self {
            radar_ok: true,
            speed_ctrl_ok: true,
            gap_ok: true,
            stop_go_ok: true,
            calibrated: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.radar_ok && self.gap_ok && self.calibrated
    }

    pub fn control_ok(&self) -> bool {
        self.speed_ctrl_ok && self.stop_go_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.calibrated || !self.radar_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.radar_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensing() {
        let c = AdaptiveCruise::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_control() {
        let c = AdaptiveCruise::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AdaptiveCruise::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = AdaptiveCruise::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_cal() {
        let mut c = AdaptiveCruise::new();
        c.calibrated = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = AdaptiveCruise::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
