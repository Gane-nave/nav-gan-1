/// wf version2: create, migrate, rollback, compare, log
/// Phase 2228

#[derive(Debug, Clone)]
pub struct WfVersion2 {
    pub create_ok: bool,
    pub migrate_ok: bool,
    pub rollback_ok: bool,
    pub compare_ok: bool,
    pub log_ok: bool,
}

impl Default for WfVersion2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfVersion2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            migrate_ok: true,
            rollback_ok: true,
            compare_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.migrate_ok && self.rollback_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.migrate_ok
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
        let c = WfVersion2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfVersion2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfVersion2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfVersion2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfVersion2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfVersion2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
