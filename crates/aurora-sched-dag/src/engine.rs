/// sched dag: add, execute, resolve, status, log
/// Phase 1743

#[derive(Debug, Clone)]
pub struct SchedDag {
    pub add_ok: bool,
    pub execute_ok: bool,
    pub resolve_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedDag {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedDag {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            execute_ok: true,
            resolve_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.execute_ok && self.resolve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = SchedDag::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedDag::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedDag::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedDag::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedDag::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedDag::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
