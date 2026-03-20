/// runtime once: call, status, reset, poison, log
/// Phase 1805

#[derive(Debug, Clone)]
pub struct RuntimeOnce {
    pub call_ok: bool,
    pub status_ok: bool,
    pub reset_ok: bool,
    pub poison_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeOnce {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeOnce {
    pub fn new() -> Self {
        Self {
            call_ok: true,
            status_ok: true,
            reset_ok: true,
            poison_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.call_ok && self.status_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.poison_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.call_ok || !self.status_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.call_ok {
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
        let c = RuntimeOnce::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeOnce::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeOnce::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeOnce::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeOnce::new();
        c.call_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeOnce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
