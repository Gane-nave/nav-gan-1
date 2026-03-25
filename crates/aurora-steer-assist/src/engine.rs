/// Steering assist: electric power steering, lane keeping, torque overlay
/// Phase 156

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SteerMode {
    Comfort,
    Normal,
    Sport,
    Parking,
}

impl SteerMode {
    pub fn assist_level(&self) -> f64 {
        match self {
            SteerMode::Comfort => 0.9,
            SteerMode::Normal => 0.7,
            SteerMode::Sport => 0.4,
            SteerMode::Parking => 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SteerAssistSystem {
    pub mode: SteerMode,
    pub steering_angle_deg: f64,
    pub torque_nm: f64,
    pub lane_offset_m: f64,
    pub lane_keeping_active: bool,
    pub speed_kmh: f64,
}

impl Default for SteerAssistSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SteerAssistSystem {
    pub fn new() -> Self {
        Self {
            mode: SteerMode::Normal,
            steering_angle_deg: 0.0,
            torque_nm: 0.0,
            lane_offset_m: 0.0,
            lane_keeping_active: true,
            speed_kmh: 0.0,
        }
    }

    pub fn assist_torque_nm(&self) -> f64 {
        self.torque_nm * self.mode.assist_level()
    }

    pub fn needs_lane_correction(&self) -> bool {
        self.lane_keeping_active && self.lane_offset_m.abs() > 0.3
    }

    pub fn correction_torque_nm(&self) -> f64 {
        if self.needs_lane_correction() {
            (self.lane_offset_m * 2.0).clamp(-5.0, 5.0)
        } else {
            0.0
        }
    }

    pub fn is_centered(&self) -> bool {
        self.lane_offset_m.abs() < 0.15
    }

    pub fn effort_score(&self) -> f64 {
        (1.0 - self.mode.assist_level()) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assist_level() {
        assert!(SteerMode::Comfort.assist_level() > SteerMode::Sport.assist_level());
    }

    #[test]
    fn test_assist_torque() {
        let mut s = SteerAssistSystem::new();
        s.torque_nm = 10.0;
        assert!(s.assist_torque_nm() > 5.0);
    }

    #[test]
    fn test_lane_correction_needed() {
        let mut s = SteerAssistSystem::new();
        s.lane_offset_m = 0.5;
        assert!(s.needs_lane_correction());
    }

    #[test]
    fn test_no_lane_correction() {
        let s = SteerAssistSystem::new();
        assert!(!s.needs_lane_correction());
    }

    #[test]
    fn test_correction_torque() {
        let mut s = SteerAssistSystem::new();
        s.lane_offset_m = 0.5;
        assert!(s.correction_torque_nm() > 0.0);
    }

    #[test]
    fn test_centered() {
        let s = SteerAssistSystem::new();
        assert!(s.is_centered());
    }

    #[test]
    fn test_effort_score() {
        let mut s = SteerAssistSystem::new();
        s.mode = SteerMode::Sport;
        assert!(s.effort_score() > 50.0);
    }

    #[test]
    fn test_parking_assist() {
        assert!((SteerMode::Parking.assist_level() - 1.0).abs() < 0.01);
    }
}
