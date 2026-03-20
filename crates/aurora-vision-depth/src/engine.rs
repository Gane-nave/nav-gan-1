/// vision depth: capture, estimate, filter, reconstruct, log
/// Phase 1476

#[derive(Debug, Clone)]
pub struct VisionDepth {
    pub capture_ok: bool,
    pub estimate_ok: bool,
    pub filter_ok: bool,
    pub reconstruct_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionDepth {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionDepth {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            estimate_ok: true,
            filter_ok: true,
            reconstruct_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.estimate_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reconstruct_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.estimate_ok
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
        let c = VisionDepth::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionDepth::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionDepth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionDepth::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionDepth::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionDepth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
