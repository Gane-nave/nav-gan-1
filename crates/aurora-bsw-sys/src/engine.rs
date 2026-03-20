/// bsw sys: scan, detect, warn, indicate, clear
/// Phase 1163

#[derive(Debug, Clone)]
pub struct BswSys {
    pub scan_ok: bool,
    pub detect_ok: bool,
    pub warn_ok: bool,
    pub indicate_ok: bool,
    pub clear_ok: bool,
}

impl Default for BswSys {
    fn default() -> Self {
        Self::new()
    }
}

impl BswSys {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            detect_ok: true,
            warn_ok: true,
            indicate_ok: true,
            clear_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.detect_ok && self.warn_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.indicate_ok && self.clear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.detect_ok
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
        let c = BswSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BswSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BswSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BswSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BswSys::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BswSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
