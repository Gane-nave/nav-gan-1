/// runtime pool2: submit, execute, resize, shutdown, log
/// Phase 1797

#[derive(Debug, Clone)]
pub struct RuntimePool2 {
    pub submit_ok: bool,
    pub execute_ok: bool,
    pub resize_ok: bool,
    pub shutdown_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimePool2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimePool2 {
    pub fn new() -> Self {
        Self {
            submit_ok: true,
            execute_ok: true,
            resize_ok: true,
            shutdown_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.submit_ok && self.execute_ok && self.resize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.shutdown_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.submit_ok || !self.execute_ok
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
        let c = RuntimePool2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimePool2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimePool2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimePool2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimePool2::new();
        c.submit_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimePool2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
