/// event hook: register, trigger, retry, disable, log
/// Phase 1873

#[derive(Debug, Clone)]
pub struct EventHook {
    pub register_ok: bool,
    pub trigger_ok: bool,
    pub retry_ok: bool,
    pub disable_ok: bool,
    pub log_ok: bool,
}

impl Default for EventHook {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHook {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            trigger_ok: true,
            retry_ok: true,
            disable_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.trigger_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.disable_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.trigger_ok
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
        let c = EventHook::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventHook::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventHook::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventHook::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventHook::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventHook::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
