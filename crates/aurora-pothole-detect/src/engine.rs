/// Pothole detection: depth, width, alert, avoidance, report
/// Phase 939

#[derive(Debug, Clone)]
pub struct PotholeDetect {
    pub depth_ok: bool,
    pub width_ok: bool,
    pub alert_ok: bool,
    pub avoidance_ok: bool,
    pub report_ok: bool,
}

impl Default for PotholeDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl PotholeDetect {
    pub fn new() -> Self {
        Self {
            depth_ok: true,
            width_ok: true,
            alert_ok: true,
            avoidance_ok: true,
            report_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.depth_ok && self.width_ok
    }

    pub fn response_ok(&self) -> bool {
        self.alert_ok && self.avoidance_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.depth_ok || !self.width_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.depth_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = PotholeDetect::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = PotholeDetect::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PotholeDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = PotholeDetect::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_depth() {
        let mut c = PotholeDetect::new();
        c.depth_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = PotholeDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
