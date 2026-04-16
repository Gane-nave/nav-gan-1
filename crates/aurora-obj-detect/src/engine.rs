/// Object detection: detect, classify, track, predict, fuse
/// Phase 1113

#[derive(Debug, Clone)]
pub struct ObjDetect {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub track_ok: bool,
    pub predict_ok: bool,
    pub fuse_ok: bool,
}

impl Default for ObjDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjDetect {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            track_ok: true,
            predict_ok: true,
            fuse_ok: true,
        }
    }

    pub fn perception_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.track_ok
    }

    pub fn prediction_ok(&self) -> bool {
        self.predict_ok && self.fuse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.perception_ok() && self.prediction_ok()
    }

    pub fn needs_retrain(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perception() {
        let c = ObjDetect::new();
        assert!(c.perception_ok());
    }

    #[test]
    fn test_prediction() {
        let c = ObjDetect::new();
        assert!(c.prediction_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObjDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retrain() {
        let c = ObjDetect::new();
        assert!(!c.needs_retrain());
    }

    #[test]
    fn test_detect() {
        let mut c = ObjDetect::new();
        c.detect_ok = false;
        assert!(c.needs_retrain());
    }

    #[test]
    fn test_health() {
        let c = ObjDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
