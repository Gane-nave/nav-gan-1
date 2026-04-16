/// sched deadline: schedule, check, extend, cancel, log
/// Phase 1738

#[derive(Debug, Clone)]
pub struct SchedDeadline {
    pub schedule_ok: bool,
    pub check_ok: bool,
    pub extend_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedDeadline {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedDeadline {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            check_ok: true,
            extend_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.check_ok && self.extend_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
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
        let c = SchedDeadline::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedDeadline::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedDeadline::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedDeadline::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedDeadline::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedDeadline::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
