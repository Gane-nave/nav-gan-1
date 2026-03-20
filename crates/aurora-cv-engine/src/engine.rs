/// CV engine: detect, segment, track, classify, reconstruct
/// Phase 1023

#[derive(Debug, Clone)]
pub struct CvEngine {
    pub detect_ok: bool,
    pub segment_ok: bool,
    pub track_ok: bool,
    pub classify_ok: bool,
    pub reconstruct_ok: bool,
}

impl Default for CvEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CvEngine {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            segment_ok: true,
            track_ok: true,
            classify_ok: true,
            reconstruct_ok: true,
        }
    }

    pub fn perception_ok(&self) -> bool {
        self.detect_ok && self.segment_ok && self.track_ok
    }

    pub fn understanding_ok(&self) -> bool {
        self.classify_ok && self.reconstruct_ok
    }

    pub fn all_ok(&self) -> bool {
        self.perception_ok() && self.understanding_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.detect_ok || !self.track_ok
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
    fn test_perception() {
        let c = CvEngine::new();
        assert!(c.perception_ok());
    }

    #[test]
    fn test_understanding() {
        let c = CvEngine::new();
        assert!(c.understanding_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CvEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = CvEngine::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_detect() {
        let mut c = CvEngine::new();
        c.detect_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = CvEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
