/// Lane detection: segment, fit, classify, predict, merge
/// Phase 1114

#[derive(Debug, Clone)]
pub struct LaneDetect {
    pub segment_ok: bool,
    pub fit_ok: bool,
    pub classify_ok: bool,
    pub predict_ok: bool,
    pub merge_ok: bool,
}

impl Default for LaneDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneDetect {
    pub fn new() -> Self {
        Self {
            segment_ok: true,
            fit_ok: true,
            classify_ok: true,
            predict_ok: true,
            merge_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.segment_ok && self.fit_ok && self.classify_ok
    }

    pub fn tracking_ok(&self) -> bool {
        self.predict_ok && self.merge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.tracking_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.segment_ok || !self.fit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.segment_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = LaneDetect::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_tracking() {
        let c = LaneDetect::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaneDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = LaneDetect::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_segment() {
        let mut c = LaneDetect::new();
        c.segment_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = LaneDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
