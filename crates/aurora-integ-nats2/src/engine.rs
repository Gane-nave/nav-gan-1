/// integ nats2: publish, subscribe, request, reply, log
/// Phase 2251

#[derive(Debug, Clone)]
pub struct IntegNats2 {
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub request_ok: bool,
    pub reply_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegNats2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegNats2 {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            subscribe_ok: true,
            request_ok: true,
            reply_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.subscribe_ok && self.request_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reply_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.publish_ok || !self.subscribe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.publish_ok {
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
        let c = IntegNats2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegNats2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegNats2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegNats2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegNats2::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegNats2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
