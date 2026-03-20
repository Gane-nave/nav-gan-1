/// sched ratelimit: acquire, release, check, reset, log
/// Phase 1739

#[derive(Debug, Clone)]
pub struct SchedRatelimit {
    pub acquire_ok: bool,
    pub release_ok: bool,
    pub check_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedRatelimit {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedRatelimit {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            release_ok: true,
            check_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.release_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.acquire_ok || !self.release_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.acquire_ok {
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
        let c = SchedRatelimit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedRatelimit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedRatelimit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedRatelimit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedRatelimit::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedRatelimit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
