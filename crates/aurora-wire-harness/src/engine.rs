/// wire harness: route, bundle, shield, ground, check
/// Phase 1266

#[derive(Debug, Clone)]
pub struct WireHarness {
    pub route_ok: bool,
    pub bundle_ok: bool,
    pub shield_ok: bool,
    pub ground_ok: bool,
    pub check_ok: bool,
}

impl Default for WireHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl WireHarness {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            bundle_ok: true,
            shield_ok: true,
            ground_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.bundle_ok && self.shield_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.ground_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.bundle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = WireHarness::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WireHarness::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WireHarness::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WireHarness::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WireHarness::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WireHarness::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
