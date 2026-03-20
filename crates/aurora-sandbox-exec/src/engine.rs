/// Sandbox executor: isolate, limit, monitor, terminate
/// Phase 1010

#[derive(Debug, Clone)]
pub struct SandboxExec {
    pub isolate_ok: bool,
    pub limit_ok: bool,
    pub monitor_ok: bool,
    pub terminate_ok: bool,
    pub policy_ok: bool,
}

impl Default for SandboxExec {
    fn default() -> Self {
        Self::new()
    }
}

impl SandboxExec {
    pub fn new() -> Self {
        Self {
            isolate_ok: true,
            limit_ok: true,
            monitor_ok: true,
            terminate_ok: true,
            policy_ok: true,
        }
    }

    pub fn containment_ok(&self) -> bool {
        self.isolate_ok && self.limit_ok && self.policy_ok
    }

    pub fn control_ok(&self) -> bool {
        self.monitor_ok && self.terminate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.containment_ok() && self.control_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.isolate_ok || !self.policy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.isolate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containment() {
        let c = SandboxExec::new();
        assert!(c.containment_ok());
    }

    #[test]
    fn test_control() {
        let c = SandboxExec::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SandboxExec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = SandboxExec::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_isolate() {
        let mut c = SandboxExec::new();
        c.isolate_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = SandboxExec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
