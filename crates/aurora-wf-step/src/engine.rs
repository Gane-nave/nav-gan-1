/// wf step: create, execute, retry, skip, log
/// Phase 2221

#[derive(Debug, Clone)]
pub struct WfStep {
    pub create_ok: bool,
    pub execute_ok: bool,
    pub retry_ok: bool,
    pub skip_ok: bool,
    pub log_ok: bool,
}

impl Default for WfStep {
    fn default() -> Self {
        Self::new()
    }
}

impl WfStep {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            execute_ok: true,
            retry_ok: true,
            skip_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.execute_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.skip_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.execute_ok
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
        let c = WfStep::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfStep::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfStep::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfStep::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfStep::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfStep::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
