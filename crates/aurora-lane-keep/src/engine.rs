/// Lane keeping assist: lane detection, steering correction, centering
/// Phase 263

#[derive(Debug, Clone)]
pub struct LaneKeepAssist {
    pub active: bool,
    pub lane_detected: bool,
    pub offset_m: f64,
    pub correction_torque_nm: f64,
    pub camera_ok: bool,
    pub hands_on_wheel: bool,
}

impl Default for LaneKeepAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneKeepAssist {
    pub fn new() -> Self {
        Self {
            active: true,
            lane_detected: true,
            offset_m: 0.0,
            correction_torque_nm: 0.0,
            camera_ok: true,
            hands_on_wheel: true,
        }
    }

    pub fn centered(&self) -> bool {
        self.offset_m.abs() < 0.2
    }

    pub fn drifting(&self) -> bool {
        self.offset_m.abs() > 0.5
    }

    pub fn can_operate(&self) -> bool {
        self.camera_ok && self.lane_detected && self.hands_on_wheel
    }

    pub fn correcting(&self) -> bool {
        self.correction_torque_nm.abs() > 0.1
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 0.0;
        }
        if !self.lane_detected {
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
        let l = LaneKeepAssist::new();
        assert!(l.centered());
    }

    #[test]
    fn test_not_drifting() {
        let l = LaneKeepAssist::new();
        assert!(!l.drifting());
    }

    #[test]
    fn test_can_operate() {
        let l = LaneKeepAssist::new();
        assert!(l.can_operate());
    }

    #[test]
    fn test_not_correcting() {
        let l = LaneKeepAssist::new();
        assert!(!l.correcting());
    }

    #[test]
    fn test_drift() {
        let mut l = LaneKeepAssist::new();
        l.offset_m = 0.8;
        assert!(l.drifting());
    }

    #[test]
    fn test_health() {
        let l = LaneKeepAssist::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
