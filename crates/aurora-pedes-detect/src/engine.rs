/// pedes detect: scan, classify, track, warn, brake
/// Phase 1329

#[derive(Debug, Clone)]
pub struct PedesDetect {
    pub scan_ok: bool,
    pub classify_ok: bool,
    pub track_ok: bool,
    pub warn_ok: bool,
    pub brake_ok: bool,
}

impl Default for PedesDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl PedesDetect {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            classify_ok: true,
            track_ok: true,
            warn_ok: true,
            brake_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.classify_ok && self.track_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.warn_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = PedesDetect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PedesDetect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PedesDetect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PedesDetect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PedesDetect::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PedesDetect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
