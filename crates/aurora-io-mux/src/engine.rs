/// io mux: register, poll, deregister, wake, log
/// Phase 1992

#[derive(Debug, Clone)]
pub struct IoMux {
    pub register_ok: bool,
    pub poll_ok: bool,
    pub deregister_ok: bool,
    pub wake_ok: bool,
    pub log_ok: bool,
}

impl Default for IoMux {
    fn default() -> Self {
        Self::new()
    }
}

impl IoMux {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            poll_ok: true,
            deregister_ok: true,
            wake_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.poll_ok && self.deregister_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wake_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.poll_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = IoMux::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoMux::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoMux::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoMux::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoMux::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoMux::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
