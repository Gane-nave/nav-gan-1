/// DR planner: assess, plan, test, failover, failback
/// Phase 1085

#[derive(Debug, Clone)]
pub struct DrPlanner {
    pub assess_ok: bool,
    pub plan_ok: bool,
    pub test_ok: bool,
    pub failover_ok: bool,
    pub failback_ok: bool,
}

impl Default for DrPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl DrPlanner {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            plan_ok: true,
            test_ok: true,
            failover_ok: true,
            failback_ok: true,
        }
    }

    pub fn preparedness_ok(&self) -> bool {
        self.assess_ok && self.plan_ok && self.test_ok
    }

    pub fn recovery_ok(&self) -> bool {
        self.failover_ok && self.failback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.preparedness_ok() && self.recovery_ok()
    }

    pub fn needs_drill(&self) -> bool {
        !self.test_ok || !self.failover_ok
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
    fn test_preparedness() {
        let c = DrPlanner::new();
        assert!(c.preparedness_ok());
    }

    #[test]
    fn test_recovery() {
        let c = DrPlanner::new();
        assert!(c.recovery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DrPlanner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_drill() {
        let c = DrPlanner::new();
        assert!(!c.needs_drill());
    }

    #[test]
    fn test_test_field() {
        let mut c = DrPlanner::new();
        c.test_ok = false;
        assert!(c.needs_drill());
    }

    #[test]
    fn test_health() {
        let c = DrPlanner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
