/// ml pipeline2: define, execute, monitor, rollback, log
/// Phase 1954

#[derive(Debug, Clone)]
pub struct MlPipeline2 {
    pub define_ok: bool,
    pub execute_ok: bool,
    pub monitor_ok: bool,
    pub rollback_ok: bool,
    pub log_ok: bool,
}

impl Default for MlPipeline2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MlPipeline2 {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            execute_ok: true,
            monitor_ok: true,
            rollback_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.execute_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rollback_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = MlPipeline2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlPipeline2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlPipeline2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlPipeline2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlPipeline2::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlPipeline2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
