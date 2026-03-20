/// sched affinity: bind, migrate, check, release, log
/// Phase 2358

#[derive(Debug, Clone)]
pub struct SchedAffinity {
    pub bind_ok: bool,
    pub migrate_ok: bool,
    pub check_ok: bool,
    pub release_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedAffinity {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedAffinity {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            migrate_ok: true,
            check_ok: true,
            release_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bind_ok && self.migrate_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.release_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bind_ok || !self.migrate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bind_ok {
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
        let c = SchedAffinity::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedAffinity::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedAffinity::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedAffinity::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedAffinity::new();
        c.bind_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedAffinity::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
