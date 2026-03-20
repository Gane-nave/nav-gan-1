/// fleet sched: plan, assign, optimize, dispatch, log
/// Phase 1411

#[derive(Debug, Clone)]
pub struct FleetSched {
    pub plan_ok: bool,
    pub assign_ok: bool,
    pub optimize_ok: bool,
    pub dispatch_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetSched {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetSched {
    pub fn new() -> Self {
        Self {
            plan_ok: true,
            assign_ok: true,
            optimize_ok: true,
            dispatch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.plan_ok && self.assign_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dispatch_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.plan_ok || !self.assign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.plan_ok {
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
        let c = FleetSched::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetSched::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetSched::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetSched::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetSched::new();
        c.plan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetSched::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
