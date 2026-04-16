/// aurora-msg-rabbit: msg rabbit
/// Phase 2593

#[derive(Debug, Clone)]
pub struct MsgRabbit {
    pub publish_ok: bool,
    pub consume_ok: bool,
    pub ack_ok: bool,
    pub dlq_ok: bool,
    pub monitor_ok: bool,
}

impl Default for MsgRabbit {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgRabbit {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            consume_ok: true,
            ack_ok: true,
            dlq_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.consume_ok && self.ack_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dlq_ok && self.monitor_ok
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
        let c = MsgRabbit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MsgRabbit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MsgRabbit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MsgRabbit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MsgRabbit::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MsgRabbit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MsgRabbit::default();
        assert!(c.all_ok());
    }
}
