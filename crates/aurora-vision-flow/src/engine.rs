/// vision flow: capture, compute, filter, interpolate, log
/// Phase 1477

#[derive(Debug, Clone)]
pub struct VisionFlow {
    pub capture_ok: bool,
    pub compute_ok: bool,
    pub filter_ok: bool,
    pub interpolate_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionFlow {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionFlow {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            compute_ok: true,
            filter_ok: true,
            interpolate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.compute_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.interpolate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.compute_ok
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
        let c = VisionFlow::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionFlow::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionFlow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionFlow::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionFlow::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionFlow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
