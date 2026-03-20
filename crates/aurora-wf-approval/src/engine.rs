/// wf approval: request, approve, reject, escalate, log
/// Phase 2233

#[derive(Debug, Clone)]
pub struct WfApproval {
    pub request_ok: bool,
    pub approve_ok: bool,
    pub reject_ok: bool,
    pub escalate_ok: bool,
    pub log_ok: bool,
}

impl Default for WfApproval {
    fn default() -> Self {
        Self::new()
    }
}

impl WfApproval {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            approve_ok: true,
            reject_ok: true,
            escalate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.approve_ok && self.reject_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.escalate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.approve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = WfApproval::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfApproval::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfApproval::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfApproval::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfApproval::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfApproval::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
