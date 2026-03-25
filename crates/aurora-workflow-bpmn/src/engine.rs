/// workflow bpmn: parse, execute, suspend, resume, log
/// Phase 1746

#[derive(Debug, Clone)]
pub struct WorkflowBpmn {
    pub parse_ok: bool,
    pub execute_ok: bool,
    pub suspend_ok: bool,
    pub resume_ok: bool,
    pub log_ok: bool,
}

impl Default for WorkflowBpmn {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowBpmn {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            execute_ok: true,
            suspend_ok: true,
            resume_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.execute_ok && self.suspend_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resume_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = WorkflowBpmn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WorkflowBpmn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WorkflowBpmn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WorkflowBpmn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WorkflowBpmn::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WorkflowBpmn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
