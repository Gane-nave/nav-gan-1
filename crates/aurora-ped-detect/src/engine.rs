/// Pedestrian detection: detect, pose, intent, predict, alert
/// Phase 1116

#[derive(Debug, Clone)]
pub struct PedDetect {
    pub detect_ok: bool,
    pub pose_ok: bool,
    pub intent_ok: bool,
    pub predict_ok: bool,
    pub alert_ok: bool,
}

impl Default for PedDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl PedDetect {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            pose_ok: true,
            intent_ok: true,
            predict_ok: true,
            alert_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.detect_ok && self.pose_ok && self.intent_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.predict_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.safety_ok()
    }

    pub fn needs_retrain(&self) -> bool {
        !self.detect_ok || !self.pose_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = PedDetect::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_safety() {
        let c = PedDetect::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retrain() {
        let c = PedDetect::new();
        assert!(!c.needs_retrain());
    }

    #[test]
    fn test_detect() {
        let mut c = PedDetect::new();
        c.detect_ok = false;
        assert!(c.needs_retrain());
    }

    #[test]
    fn test_health() {
        let c = PedDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
