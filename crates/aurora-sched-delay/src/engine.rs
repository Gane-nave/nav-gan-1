/// sched delay: schedule, cancel, status, flush, log
/// Phase 1736

#[derive(Debug, Clone)]
pub struct SchedDelay {
    pub schedule_ok: bool,
    pub cancel_ok: bool,
    pub status_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedDelay {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedDelay {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            cancel_ok: true,
            status_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.cancel_ok && self.status_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.cancel_ok
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
        let c = SchedDelay::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedDelay::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedDelay::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedDelay::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedDelay::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedDelay::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
