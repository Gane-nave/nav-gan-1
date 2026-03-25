/// rt green: spawn, schedule, preempt, terminate, log
/// Phase 2333

#[derive(Debug, Clone)]
pub struct RtGreen {
    pub spawn_ok: bool,
    pub schedule_ok: bool,
    pub preempt_ok: bool,
    pub terminate_ok: bool,
    pub log_ok: bool,
}

impl Default for RtGreen {
    fn default() -> Self {
        Self::new()
    }
}

impl RtGreen {
    pub fn new() -> Self {
        Self {
            spawn_ok: true,
            schedule_ok: true,
            preempt_ok: true,
            terminate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spawn_ok && self.schedule_ok && self.preempt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.terminate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spawn_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spawn_ok {
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
        let c = RtGreen::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtGreen::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtGreen::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtGreen::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtGreen::new();
        c.spawn_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtGreen::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
