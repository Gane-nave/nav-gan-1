/// Lane departure warning: boundary detection, vibration alert, visual warning
/// Phase 264

#[derive(Debug, Clone)]
pub struct LaneDepartureWarning {
    pub active: bool,
    pub left_line_detected: bool,
    pub right_line_detected: bool,
    pub departing_left: bool,
    pub departing_right: bool,
    pub camera_ok: bool,
}

impl Default for LaneDepartureWarning {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneDepartureWarning {
    pub fn new() -> Self {
        Self {
            active: true,
            left_line_detected: true,
            right_line_detected: true,
            departing_left: false,
            departing_right: false,
            camera_ok: true,
        }
    }

    pub fn lines_detected(&self) -> bool {
        self.left_line_detected || self.right_line_detected
    }

    pub fn warning_active(&self) -> bool {
        self.departing_left || self.departing_right
    }

    pub fn can_operate(&self) -> bool {
        self.camera_ok && self.lines_detected()
    }

    pub fn safe_in_lane(&self) -> bool {
        !self.departing_left && !self.departing_right
    }

    pub fn health_score(&self) -> f64 {
        if !self.camera_ok {
            return 0.0;
        }
        if !self.lines_detected() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_detected() {
        let l = LaneDepartureWarning::new();
        assert!(l.lines_detected());
    }

    #[test]
    fn test_no_warning() {
        let l = LaneDepartureWarning::new();
        assert!(!l.warning_active());
    }

    #[test]
    fn test_can_operate() {
        let l = LaneDepartureWarning::new();
        assert!(l.can_operate());
    }

    #[test]
    fn test_safe() {
        let l = LaneDepartureWarning::new();
        assert!(l.safe_in_lane());
    }

    #[test]
    fn test_departing() {
        let mut l = LaneDepartureWarning::new();
        l.departing_left = true;
        assert!(l.warning_active());
    }

    #[test]
    fn test_health() {
        let l = LaneDepartureWarning::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
