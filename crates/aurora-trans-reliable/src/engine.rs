/// trans reliable: send, ack, retry, order, log
/// Phase 2274

#[derive(Debug, Clone)]
pub struct TransReliable {
    pub send_ok: bool,
    pub ack_ok: bool,
    pub retry_ok: bool,
    pub order_ok: bool,
    pub log_ok: bool,
}

impl Default for TransReliable {
    fn default() -> Self {
        Self::new()
    }
}

impl TransReliable {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            ack_ok: true,
            retry_ok: true,
            order_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.ack_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.order_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.ack_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.send_ok {
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
        let c = TransReliable::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransReliable::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransReliable::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransReliable::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransReliable::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransReliable::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
