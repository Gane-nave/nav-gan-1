/// Highway merge: gap, speed, acceleration, signal
/// Phase 935

#[derive(Debug, Clone)]
pub struct HighwayMerge {
    pub gap_ok: bool,
    pub speed_ok: bool,
    pub accel_ok: bool,
    pub signal_ok: bool,
    pub radar_ok: bool,
}

impl Default for HighwayMerge {
    fn default() -> Self {
        Self::new()
    }
}

impl HighwayMerge {
    pub fn new() -> Self {
        Self {
            gap_ok: true,
            speed_ok: true,
            accel_ok: true,
            signal_ok: true,
            radar_ok: true,
        }
    }

    pub fn planning_ok(&self) -> bool {
        self.gap_ok && self.speed_ok && self.radar_ok
    }

    pub fn execution_ok(&self) -> bool {
        self.accel_ok && self.signal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.planning_ok() && self.execution_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.radar_ok || !self.gap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.radar_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let c = HighwayMerge::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_execution() {
        let c = HighwayMerge::new();
        assert!(c.execution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HighwayMerge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = HighwayMerge::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_radar() {
        let mut c = HighwayMerge::new();
        c.radar_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = HighwayMerge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
