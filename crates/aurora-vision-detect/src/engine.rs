/// vision detect: capture, preprocess, detect, track, log
/// Phase 1474

#[derive(Debug, Clone)]
pub struct VisionDetect {
    pub capture_ok: bool,
    pub preprocess_ok: bool,
    pub detect_ok: bool,
    pub track_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionDetect {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            preprocess_ok: true,
            detect_ok: true,
            track_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.preprocess_ok && self.detect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.track_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.preprocess_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VisionDetect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionDetect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionDetect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionDetect::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
