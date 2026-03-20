/// sched interval: start, stop, reset, status, log
/// Phase 1735

#[derive(Debug, Clone)]
pub struct SchedInterval {
    pub start_ok: bool,
    pub stop_ok: bool,
    pub reset_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedInterval {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            stop_ok: true,
            reset_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.stop_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.stop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = SchedInterval::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedInterval::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedInterval::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedInterval::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedInterval::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedInterval::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
