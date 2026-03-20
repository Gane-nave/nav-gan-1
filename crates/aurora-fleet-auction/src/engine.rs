/// fleet auction: list, bid, award, settle, log
/// Phase 1424

#[derive(Debug, Clone)]
pub struct FleetAuction {
    pub list_ok: bool,
    pub bid_ok: bool,
    pub award_ok: bool,
    pub settle_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetAuction {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetAuction {
    pub fn new() -> Self {
        Self {
            list_ok: true,
            bid_ok: true,
            award_ok: true,
            settle_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.list_ok && self.bid_ok && self.award_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.settle_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.list_ok || !self.bid_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.list_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetAuction::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetAuction::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetAuction::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetAuction::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetAuction::new();
        c.list_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetAuction::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
