/// cloud lb: route, balance, health, failover, log
/// Phase 1460

#[derive(Debug, Clone)]
pub struct CloudLb {
    pub route_ok: bool,
    pub balance_ok: bool,
    pub health_ok: bool,
    pub failover_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudLb {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudLb {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            balance_ok: true,
            health_ok: true,
            failover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.balance_ok && self.health_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.failover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudLb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudLb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudLb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudLb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudLb::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudLb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
