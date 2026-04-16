/// wf state2: save, load, migrate, compact, log
/// Phase 2227

#[derive(Debug, Clone)]
pub struct WfState2 {
    pub save_ok: bool,
    pub load_ok: bool,
    pub migrate_ok: bool,
    pub compact_ok: bool,
    pub log_ok: bool,
}

impl Default for WfState2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfState2 {
    pub fn new() -> Self {
        Self {
            save_ok: true,
            load_ok: true,
            migrate_ok: true,
            compact_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.save_ok && self.load_ok && self.migrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.save_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.save_ok {
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
        let c = WfState2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfState2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfState2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfState2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfState2::new();
        c.save_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfState2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
