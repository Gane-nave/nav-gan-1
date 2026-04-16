/// wf loop: init, iterate, check, complete, log
/// Phase 2224

#[derive(Debug, Clone)]
pub struct WfLoop {
    pub init_ok: bool,
    pub iterate_ok: bool,
    pub check_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for WfLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl WfLoop {
    pub fn new() -> Self {
        Self {
            init_ok: true,
            iterate_ok: true,
            check_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.init_ok && self.iterate_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.init_ok || !self.iterate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.init_ok {
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
        let c = WfLoop::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfLoop::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfLoop::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfLoop::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfLoop::new();
        c.init_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfLoop::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
