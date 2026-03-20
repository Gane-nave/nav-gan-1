/// road surface: scan, classify, grip, warn, log
/// Phase 1337

#[derive(Debug, Clone)]
pub struct RoadSurface {
    pub scan_ok: bool,
    pub classify_ok: bool,
    pub grip_ok: bool,
    pub warn_ok: bool,
    pub log_ok: bool,
}

impl Default for RoadSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadSurface {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            classify_ok: true,
            grip_ok: true,
            warn_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.classify_ok && self.grip_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.warn_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
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
        let c = RoadSurface::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RoadSurface::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoadSurface::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RoadSurface::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RoadSurface::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RoadSurface::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
