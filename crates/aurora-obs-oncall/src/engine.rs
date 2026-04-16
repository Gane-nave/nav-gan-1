/// obs oncall: schedule, notify, escalate, handoff, log
/// Phase 2165

#[derive(Debug, Clone)]
pub struct ObsOncall {
    pub schedule_ok: bool,
    pub notify_ok: bool,
    pub escalate_ok: bool,
    pub handoff_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsOncall {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsOncall {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            notify_ok: true,
            escalate_ok: true,
            handoff_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.notify_ok && self.escalate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.handoff_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.notify_ok
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
        let c = ObsOncall::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsOncall::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsOncall::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsOncall::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsOncall::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsOncall::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
