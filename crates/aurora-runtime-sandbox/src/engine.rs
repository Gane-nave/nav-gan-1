/// runtime sandbox: create, execute, limit, destroy, log
/// Phase 1792

#[derive(Debug, Clone)]
pub struct RuntimeSandbox {
    pub create_ok: bool,
    pub execute_ok: bool,
    pub limit_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeSandbox {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            execute_ok: true,
            limit_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.execute_ok && self.limit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
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
        let c = RuntimeSandbox::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeSandbox::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeSandbox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeSandbox::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeSandbox::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeSandbox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
