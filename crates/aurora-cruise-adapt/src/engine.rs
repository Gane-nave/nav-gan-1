/// Adaptive cruise: radar, follow, gap, stop-go
/// Phase 927

#[derive(Debug, Clone)]
pub struct CruiseAdapt {
    pub radar_ok: bool,
    pub follow_ok: bool,
    pub gap_ok: bool,
    pub stop_go_ok: bool,
    pub speed_ok: bool,
}

impl Default for CruiseAdapt {
    fn default() -> Self {
        Self::new()
    }
}

impl CruiseAdapt {
    pub fn new() -> Self {
        Self {
            radar_ok: true,
            follow_ok: true,
            gap_ok: true,
            stop_go_ok: true,
            speed_ok: true,
        }
    }

    pub fn sensing_ok(&self) -> bool {
        self.radar_ok && self.speed_ok
    }

    pub fn control_ok(&self) -> bool {
        self.follow_ok && self.gap_ok && self.stop_go_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sensing_ok() && self.control_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.radar_ok || !self.speed_ok
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
        let c = CruiseAdapt::new();
        assert!(c.sensing_ok());
    }

    #[test]
    fn test_control() {
        let c = CruiseAdapt::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CruiseAdapt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = CruiseAdapt::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_radar() {
        let mut c = CruiseAdapt::new();
        c.radar_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = CruiseAdapt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
