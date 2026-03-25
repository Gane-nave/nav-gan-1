/// runtime condvar: wait, notify, broadcast, timeout, log
/// Phase 1804

#[derive(Debug, Clone)]
pub struct RuntimeCondvar {
    pub wait_ok: bool,
    pub notify_ok: bool,
    pub broadcast_ok: bool,
    pub timeout_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeCondvar {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeCondvar {
    pub fn new() -> Self {
        Self {
            wait_ok: true,
            notify_ok: true,
            broadcast_ok: true,
            timeout_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.wait_ok && self.notify_ok && self.broadcast_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.timeout_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.wait_ok || !self.notify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wait_ok {
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
        let c = RuntimeCondvar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeCondvar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeCondvar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeCondvar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeCondvar::new();
        c.wait_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeCondvar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
