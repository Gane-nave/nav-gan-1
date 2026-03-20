/// Gesture UI: hand tracking, swipe, pinch, point, wave
/// Phase 897

#[derive(Debug, Clone)]
pub struct GestureUi {
    pub tracking_ok: bool,
    pub swipe_ok: bool,
    pub pinch_ok: bool,
    pub point_ok: bool,
    pub wave_ok: bool,
}

impl Default for GestureUi {
    fn default() -> Self {
        Self::new()
    }
}

impl GestureUi {
    pub fn new() -> Self {
        Self {
            tracking_ok: true,
            swipe_ok: true,
            pinch_ok: true,
            point_ok: true,
            wave_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.tracking_ok && self.swipe_ok && self.pinch_ok
    }

    pub fn interaction_ok(&self) -> bool {
        self.point_ok && self.wave_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.interaction_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.tracking_ok || !self.swipe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tracking_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = GestureUi::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_interaction() {
        let c = GestureUi::new();
        assert!(c.interaction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GestureUi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = GestureUi::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_tracking() {
        let mut c = GestureUi::new();
        c.tracking_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = GestureUi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
