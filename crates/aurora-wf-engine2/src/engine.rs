/// wf engine2: define, execute, monitor, retry, log
/// Phase 2220

#[derive(Debug, Clone)]
pub struct WfEngine2 {
    pub define_ok: bool,
    pub execute_ok: bool,
    pub monitor_ok: bool,
    pub retry_ok: bool,
    pub log_ok: bool,
}

impl Default for WfEngine2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfEngine2 {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            execute_ok: true,
            monitor_ok: true,
            retry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.execute_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.log_ok
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
        let c = WfEngine2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfEngine2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfEngine2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfEngine2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfEngine2::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfEngine2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
