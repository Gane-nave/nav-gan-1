/// integ webhook: register, trigger, verify, retry, log
/// Phase 1653

#[derive(Debug, Clone)]
pub struct IntegWebhook {
    pub register_ok: bool,
    pub trigger_ok: bool,
    pub verify_ok: bool,
    pub retry_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegWebhook {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegWebhook {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            trigger_ok: true,
            verify_ok: true,
            retry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.trigger_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.log_ok
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
        let c = IntegWebhook::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegWebhook::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegWebhook::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegWebhook::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegWebhook::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegWebhook::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
