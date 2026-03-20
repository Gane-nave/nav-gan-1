/// fleet route: plan, optimize, navigate, adapt, log
/// Phase 1416

#[derive(Debug, Clone)]
pub struct FleetRoute {
    pub plan_ok: bool,
    pub optimize_ok: bool,
    pub navigate_ok: bool,
    pub adapt_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetRoute {
    pub fn new() -> Self {
        Self {
            plan_ok: true,
            optimize_ok: true,
            navigate_ok: true,
            adapt_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.plan_ok && self.optimize_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.plan_ok || !self.optimize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.plan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetRoute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetRoute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetRoute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetRoute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetRoute::new();
        c.plan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetRoute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
