/// Haptic seat: vibration, pattern, navigation, alert
/// Phase 899

#[derive(Debug, Clone)]
pub struct HapticSeat {
    pub vibration_ok: bool,
    pub pattern_ok: bool,
    pub nav_ok: bool,
    pub alert_ok: bool,
    pub intensity_ok: bool,
}

impl Default for HapticSeat {
    fn default() -> Self {
        Self::new()
    }
}

impl HapticSeat {
    pub fn new() -> Self {
        Self {
            vibration_ok: true,
            pattern_ok: true,
            nav_ok: true,
            alert_ok: true,
            intensity_ok: true,
        }
    }

    pub fn feedback_ok(&self) -> bool {
        self.vibration_ok && self.pattern_ok && self.intensity_ok
    }

    pub fn integration_ok(&self) -> bool {
        self.nav_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.feedback_ok() && self.integration_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.vibration_ok || !self.pattern_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vibration_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback() {
        let c = HapticSeat::new();
        assert!(c.feedback_ok());
    }

    #[test]
    fn test_integration() {
        let c = HapticSeat::new();
        assert!(c.integration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HapticSeat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = HapticSeat::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_vibration() {
        let mut c = HapticSeat::new();
        c.vibration_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = HapticSeat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
