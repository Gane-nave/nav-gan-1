/// rt wasm2: compile, instantiate, execute, destroy, log
/// Phase 2342

#[derive(Debug, Clone)]
pub struct RtWasm2 {
    pub compile_ok: bool,
    pub instantiate_ok: bool,
    pub execute_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for RtWasm2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtWasm2 {
    pub fn new() -> Self {
        Self {
            compile_ok: true,
            instantiate_ok: true,
            execute_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compile_ok && self.instantiate_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compile_ok || !self.instantiate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compile_ok {
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
        let c = RtWasm2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtWasm2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtWasm2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtWasm2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtWasm2::new();
        c.compile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtWasm2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
