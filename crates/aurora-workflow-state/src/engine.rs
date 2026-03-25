/// workflow state: create, transition, query, reset, log
/// Phase 1747

#[derive(Debug, Clone)]
pub struct WorkflowState {
    pub create_ok: bool,
    pub transition_ok: bool,
    pub query_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for WorkflowState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowState {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            transition_ok: true,
            query_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.transition_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.transition_ok
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
        let c = WorkflowState::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WorkflowState::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WorkflowState::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WorkflowState::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WorkflowState::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WorkflowState::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
