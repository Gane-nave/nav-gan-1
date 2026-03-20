/// vision slam: capture, extract, match, optimize, log
/// Phase 1478

#[derive(Debug, Clone)]
pub struct VisionSlam {
    pub capture_ok: bool,
    pub extract_ok: bool,
    pub match_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionSlam {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionSlam {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            extract_ok: true,
            match_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.extract_ok && self.match_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.extract_ok
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
        let c = VisionSlam::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionSlam::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionSlam::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionSlam::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionSlam::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionSlam::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
