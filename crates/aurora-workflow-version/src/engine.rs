/// workflow version: create, compare, merge, rollback, log
/// Phase 1751

#[derive(Debug, Clone)]
pub struct WorkflowVersion {
    pub create_ok: bool,
    pub compare_ok: bool,
    pub merge_ok: bool,
    pub rollback_ok: bool,
    pub log_ok: bool,
}

impl Default for WorkflowVersion {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowVersion {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            compare_ok: true,
            merge_ok: true,
            rollback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.compare_ok && self.merge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.compare_ok
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
        let c = WorkflowVersion::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WorkflowVersion::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WorkflowVersion::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WorkflowVersion::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WorkflowVersion::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WorkflowVersion::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
