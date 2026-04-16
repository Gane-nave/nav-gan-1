/// Reverse light: backup lamp, reverse camera trigger, proximity warning
/// Phase 247

#[derive(Debug, Clone)]
pub struct ReverseLight {
    pub left_ok: bool,
    pub right_ok: bool,
    pub active: bool,
    pub camera_triggered: bool,
    pub gear_in_reverse: bool,
}

impl Default for ReverseLight {
    fn default() -> Self {
        Self::new()
    }
}

impl ReverseLight {
    pub fn new() -> Self {
        Self {
            left_ok: true,
            right_ok: true,
            active: false,
            camera_triggered: false,
            gear_in_reverse: false,
        }
    }

    pub fn both_ok(&self) -> bool {
        self.left_ok && self.right_ok
    }

    pub fn should_activate(&self) -> bool {
        self.gear_in_reverse
    }

    pub fn camera_active(&self) -> bool {
        self.gear_in_reverse && self.camera_triggered
    }

    pub fn bulb_out(&self) -> bool {
        !self.left_ok || !self.right_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.left_ok && !self.right_ok {
            return 0.0;
        }
        if self.bulb_out() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_ok() {
        let r = ReverseLight::new();
        assert!(r.both_ok());
    }

    #[test]
    fn test_not_reverse() {
        let r = ReverseLight::new();
        assert!(!r.should_activate());
    }

    #[test]
    fn test_no_camera() {
        let r = ReverseLight::new();
        assert!(!r.camera_active());
    }

    #[test]
    fn test_no_bulb_out() {
        let r = ReverseLight::new();
        assert!(!r.bulb_out());
    }

    #[test]
    fn test_reverse() {
        let mut r = ReverseLight::new();
        r.gear_in_reverse = true;
        assert!(r.should_activate());
    }

    #[test]
    fn test_health() {
        let r = ReverseLight::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
