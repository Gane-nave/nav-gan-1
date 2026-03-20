/// vision calib: capture, detect, estimate, verify, log
/// Phase 1479

#[derive(Debug, Clone)]
pub struct VisionCalib {
    pub capture_ok: bool,
    pub detect_ok: bool,
    pub estimate_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionCalib {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionCalib {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            detect_ok: true,
            estimate_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.detect_ok && self.estimate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VisionCalib::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionCalib::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionCalib::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionCalib::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionCalib::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionCalib::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
