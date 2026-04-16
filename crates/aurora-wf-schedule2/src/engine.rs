/// wf schedule2: cron, interval, delay, cancel, log
/// Phase 2232

#[derive(Debug, Clone)]
pub struct WfSchedule2 {
    pub cron_ok: bool,
    pub interval_ok: bool,
    pub delay_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for WfSchedule2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfSchedule2 {
    pub fn new() -> Self {
        Self {
            cron_ok: true,
            interval_ok: true,
            delay_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.cron_ok && self.interval_ok && self.delay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.cron_ok || !self.interval_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cron_ok {
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
        let c = WfSchedule2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfSchedule2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfSchedule2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfSchedule2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfSchedule2::new();
        c.cron_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfSchedule2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
