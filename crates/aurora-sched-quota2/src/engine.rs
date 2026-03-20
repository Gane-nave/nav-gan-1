/// sched quota2: allocate, consume, check, replenish, log
/// Phase 2357

#[derive(Debug, Clone)]
pub struct SchedQuota2 {
    pub allocate_ok: bool,
    pub consume_ok: bool,
    pub check_ok: bool,
    pub replenish_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedQuota2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedQuota2 {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            consume_ok: true,
            check_ok: true,
            replenish_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.consume_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replenish_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.consume_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.allocate_ok {
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
        let c = SchedQuota2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedQuota2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedQuota2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedQuota2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedQuota2::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedQuota2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
