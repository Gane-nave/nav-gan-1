/// fleet insure: assess, quote, bind, claim, log
/// Phase 1422

#[derive(Debug, Clone)]
pub struct FleetInsure {
    pub assess_ok: bool,
    pub quote_ok: bool,
    pub bind_ok: bool,
    pub claim_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetInsure {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetInsure {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            quote_ok: true,
            bind_ok: true,
            claim_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.quote_ok && self.bind_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.claim_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.quote_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assess_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetInsure::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetInsure::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetInsure::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetInsure::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetInsure::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetInsure::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
