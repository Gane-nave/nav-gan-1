/// aurora-msg-pubsub: msg pubsub
/// Phase 2598

#[derive(Debug, Clone)]
pub struct MsgPubsub {
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub filter_ok: bool,
    pub ack_ok: bool,
    pub monitor_ok: bool,
}

impl Default for MsgPubsub {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgPubsub {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            subscribe_ok: true,
            filter_ok: true,
            ack_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.subscribe_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.ack_ok && self.monitor_ok
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
        let c = MsgPubsub::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MsgPubsub::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MsgPubsub::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MsgPubsub::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MsgPubsub::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MsgPubsub::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MsgPubsub::default();
        assert!(c.all_ok());
    }
}
