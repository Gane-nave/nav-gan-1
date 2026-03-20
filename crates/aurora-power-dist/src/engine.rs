/// power dist: route, switch, protect, balance, log
/// Phase 1366

#[derive(Debug, Clone)]
pub struct PowerDist {
    pub route_ok: bool,
    pub switch_ok: bool,
    pub protect_ok: bool,
    pub balance_ok: bool,
    pub log_ok: bool,
}

impl Default for PowerDist {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerDist {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            switch_ok: true,
            protect_ok: true,
            balance_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.switch_ok && self.protect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.switch_ok
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
        let c = PowerDist::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PowerDist::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerDist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PowerDist::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PowerDist::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PowerDist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
