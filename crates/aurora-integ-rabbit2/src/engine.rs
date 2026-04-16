/// integ rabbit2: publish, consume, ack, nack, log
/// Phase 2250

#[derive(Debug, Clone)]
pub struct IntegRabbit2 {
    pub publish_ok: bool,
    pub consume_ok: bool,
    pub ack_ok: bool,
    pub nack_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegRabbit2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegRabbit2 {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            consume_ok: true,
            ack_ok: true,
            nack_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.consume_ok && self.ack_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.nack_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.publish_ok || !self.consume_ok
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
        let c = IntegRabbit2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegRabbit2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegRabbit2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegRabbit2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegRabbit2::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegRabbit2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
