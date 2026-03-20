/// monitor jit: compile, optimize, deoptimize, cache, log
/// Phase 1579

#[derive(Debug, Clone)]
pub struct MonitorJit {
    pub compile_ok: bool,
    pub optimize_ok: bool,
    pub deoptimize_ok: bool,
    pub cache_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorJit {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorJit {
    pub fn new() -> Self {
        Self {
            compile_ok: true,
            optimize_ok: true,
            deoptimize_ok: true,
            cache_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.compile_ok && self.optimize_ok && self.deoptimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cache_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.compile_ok || !self.optimize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compile_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorJit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorJit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorJit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorJit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorJit::new();
        c.compile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorJit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
