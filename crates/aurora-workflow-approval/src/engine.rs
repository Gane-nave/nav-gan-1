/// workflow approval: submit, approve, reject, escalate, log
/// Phase 1748

#[derive(Debug, Clone)]
pub struct WorkflowApproval {
    pub submit_ok: bool,
    pub approve_ok: bool,
    pub reject_ok: bool,
    pub escalate_ok: bool,
    pub log_ok: bool,
}

impl Default for WorkflowApproval {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowApproval {
    pub fn new() -> Self {
        Self {
            submit_ok: true,
            approve_ok: true,
            reject_ok: true,
            escalate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.submit_ok && self.approve_ok && self.reject_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.escalate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.submit_ok || !self.approve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.submit_ok {
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
        let c = WorkflowApproval::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WorkflowApproval::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WorkflowApproval::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WorkflowApproval::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WorkflowApproval::new();
        c.submit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WorkflowApproval::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
