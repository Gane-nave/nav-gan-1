/// sched coop: yield_point, check, schedule, resume, log
/// Phase 2361

#[derive(Debug, Clone)]
pub struct SchedCoop {
    pub yield_point_ok: bool,
    pub check_ok: bool,
    pub schedule_ok: bool,
    pub resume_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedCoop {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedCoop {
    pub fn new() -> Self {
        Self {
            yield_point_ok: true,
            check_ok: true,
            schedule_ok: true,
            resume_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.yield_point_ok && self.check_ok && self.schedule_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resume_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.yield_point_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.yield_point_ok {
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
        let c = SchedCoop::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedCoop::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedCoop::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedCoop::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedCoop::new();
        c.yield_point_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedCoop::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
