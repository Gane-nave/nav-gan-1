/// fleet lease: negotiate, sign, manage, return, log
/// Phase 1423

#[derive(Debug, Clone)]
pub struct FleetLease {
    pub negotiate_ok: bool,
    pub sign_ok: bool,
    pub manage_ok: bool,
    pub return_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetLease {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetLease {
    pub fn new() -> Self {
        Self {
            negotiate_ok: true,
            sign_ok: true,
            manage_ok: true,
            return_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.negotiate_ok && self.sign_ok && self.manage_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.return_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.negotiate_ok || !self.sign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.negotiate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetLease::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetLease::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetLease::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetLease::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetLease::new();
        c.negotiate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetLease::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
