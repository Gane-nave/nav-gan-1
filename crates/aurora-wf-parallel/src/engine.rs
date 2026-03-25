/// wf parallel: fork, execute, join, cancel, log
/// Phase 2223

#[derive(Debug, Clone)]
pub struct WfParallel {
    pub fork_ok: bool,
    pub execute_ok: bool,
    pub join_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for WfParallel {
    fn default() -> Self {
        Self::new()
    }
}

impl WfParallel {
    pub fn new() -> Self {
        Self {
            fork_ok: true,
            execute_ok: true,
            join_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fork_ok && self.execute_ok && self.join_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fork_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fork_ok {
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
        let c = WfParallel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfParallel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfParallel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfParallel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfParallel::new();
        c.fork_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfParallel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
