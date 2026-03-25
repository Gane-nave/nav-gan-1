/// integ nats: connect, publish, subscribe, request, log
/// Phase 1651

#[derive(Debug, Clone)]
pub struct IntegNats {
    pub connect_ok: bool,
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub request_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegNats {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegNats {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            publish_ok: true,
            subscribe_ok: true,
            request_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.publish_ok && self.subscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.request_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.publish_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = IntegNats::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegNats::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegNats::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegNats::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegNats::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegNats::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
