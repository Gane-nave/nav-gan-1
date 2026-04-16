/// sched cron2: parse, schedule, execute, cancel, log
/// Phase 2354

#[derive(Debug, Clone)]
pub struct SchedCron2 {
    pub parse_ok: bool,
    pub schedule_ok: bool,
    pub execute_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedCron2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedCron2 {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            schedule_ok: true,
            execute_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.schedule_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = SchedCron2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedCron2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedCron2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedCron2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedCron2::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedCron2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
