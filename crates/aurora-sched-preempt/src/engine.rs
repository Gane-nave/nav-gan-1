/// sched preempt: check, save, switch, restore, log
/// Phase 2360

#[derive(Debug, Clone)]
pub struct SchedPreempt {
    pub check_ok: bool,
    pub save_ok: bool,
    pub switch_ok: bool,
    pub restore_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedPreempt {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedPreempt {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            save_ok: true,
            switch_ok: true,
            restore_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.save_ok && self.switch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.restore_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.save_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = SchedPreempt::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedPreempt::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedPreempt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedPreempt::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedPreempt::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedPreempt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
