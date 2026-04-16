/// event schedule: create, trigger, cancel, reschedule, log
/// Phase 1874

#[derive(Debug, Clone)]
pub struct EventSchedule {
    pub create_ok: bool,
    pub trigger_ok: bool,
    pub cancel_ok: bool,
    pub reschedule_ok: bool,
    pub log_ok: bool,
}

impl Default for EventSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSchedule {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            trigger_ok: true,
            cancel_ok: true,
            reschedule_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.trigger_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reschedule_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.trigger_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = EventSchedule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSchedule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSchedule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSchedule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSchedule::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSchedule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
