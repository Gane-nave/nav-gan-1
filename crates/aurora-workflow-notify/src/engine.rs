/// workflow notify: send, schedule, cancel, status, log
/// Phase 1749

#[derive(Debug, Clone)]
pub struct WorkflowNotify {
    pub send_ok: bool,
    pub schedule_ok: bool,
    pub cancel_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for WorkflowNotify {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowNotify {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            schedule_ok: true,
            cancel_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.schedule_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.send_ok {
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
        let c = WorkflowNotify::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WorkflowNotify::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WorkflowNotify::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WorkflowNotify::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WorkflowNotify::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WorkflowNotify::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
