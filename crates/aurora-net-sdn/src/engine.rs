/// net sdn: configure, route, monitor, optimize, log
/// Phase 2270

#[derive(Debug, Clone)]
pub struct NetSdn {
    pub configure_ok: bool,
    pub route_ok: bool,
    pub monitor_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for NetSdn {
    fn default() -> Self {
        Self::new()
    }
}

impl NetSdn {
    pub fn new() -> Self {
        Self {
            configure_ok: true,
            route_ok: true,
            monitor_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.configure_ok && self.route_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.configure_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.configure_ok {
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
        let c = NetSdn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetSdn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetSdn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetSdn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetSdn::new();
        c.configure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetSdn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
