/// sched fair2: submit, schedule, preempt, balance, log
/// Phase 2351

#[derive(Debug, Clone)]
pub struct SchedFair2 {
    pub submit_ok: bool,
    pub schedule_ok: bool,
    pub preempt_ok: bool,
    pub balance_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedFair2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedFair2 {
    pub fn new() -> Self {
        Self {
            submit_ok: true,
            schedule_ok: true,
            preempt_ok: true,
            balance_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.submit_ok && self.schedule_ok && self.preempt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.submit_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.submit_ok {
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
        let c = SchedFair2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedFair2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedFair2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedFair2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedFair2::new();
        c.submit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedFair2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
