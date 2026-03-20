/// rt jit: compile, optimize, execute, invalidate, log
/// Phase 2343

#[derive(Debug, Clone)]
pub struct RtJit {
    pub compile_ok: bool,
    pub optimize_ok: bool,
    pub execute_ok: bool,
    pub invalidate_ok: bool,
    pub log_ok: bool,
}

impl Default for RtJit {
    fn default() -> Self {
        Self::new()
    }
}

impl RtJit {
    pub fn new() -> Self {
        Self {
            compile_ok: true,
            optimize_ok: true,
            execute_ok: true,
            invalidate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compile_ok && self.optimize_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.invalidate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compile_ok || !self.optimize_ok
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
        let c = RtJit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtJit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtJit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtJit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtJit::new();
        c.compile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtJit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
