/// Driver monitoring: drowsiness detection, attention tracking, fatigue scoring
/// Phase 272

#[derive(Debug, Clone)]
pub struct DriverMonitor {
    pub eyes_open: bool,
    pub head_pose_ok: bool,
    pub attention_score: f64,
    pub drowsiness_level: u8,
    pub distracted: bool,
    pub camera_ok: bool,
}

impl Default for DriverMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverMonitor {
    pub fn new() -> Self {
        Self {
            eyes_open: true,
            head_pose_ok: true,
            attention_score: 95.0,
            drowsiness_level: 0,
            distracted: false,
            camera_ok: true,
        }
    }

    pub fn alert(&self) -> bool {
        self.eyes_open && self.head_pose_ok && !self.distracted
    }

    pub fn drowsy(&self) -> bool {
        self.drowsiness_level >= 3
    }

    pub fn needs_break(&self) -> bool {
        self.drowsiness_level >= 2 || self.attention_score < 50.0
    }

    pub fn emergency_alert(&self) -> bool {
        !self.eyes_open && self.drowsiness_level >= 4
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 0.0;
        }
        self.attention_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert() {
        let d = DriverMonitor::new();
        assert!(d.alert());
    }

    #[test]
    fn test_not_drowsy() {
        let d = DriverMonitor::new();
        assert!(!d.drowsy());
    }

    #[test]
    fn test_no_break() {
        let d = DriverMonitor::new();
        assert!(!d.needs_break());
    }

    #[test]
    fn test_no_emergency() {
        let d = DriverMonitor::new();
        assert!(!d.emergency_alert());
    }

    #[test]
    fn test_drowsy() {
        let mut d = DriverMonitor::new();
        d.drowsiness_level = 4;
        assert!(d.drowsy());
    }

    #[test]
    fn test_health() {
        let d = DriverMonitor::new();
        assert!((d.health_score() - 95.0).abs() < 0.1);
    }
}
