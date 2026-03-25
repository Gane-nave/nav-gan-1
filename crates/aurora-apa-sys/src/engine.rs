/// apa sys: scan, select, steer, park, confirm
/// Phase 1171

#[derive(Debug, Clone)]
pub struct ApaSys {
    pub scan_ok: bool,
    pub select_ok: bool,
    pub steer_ok: bool,
    pub park_ok: bool,
    pub confirm_ok: bool,
}

impl Default for ApaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl ApaSys {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            select_ok: true,
            steer_ok: true,
            park_ok: true,
            confirm_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.select_ok && self.steer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.park_ok && self.confirm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.select_ok
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
        let c = ApaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApaSys::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
