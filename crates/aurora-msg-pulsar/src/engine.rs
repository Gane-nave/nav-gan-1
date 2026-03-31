/// aurora-msg-pulsar: msg pulsar
/// Phase 2595

#[derive(Debug, Clone)]
pub struct MsgPulsar {
    pub produce_ok: bool,
    pub consume_ok: bool,
    pub ack_ok: bool,
    pub schema_ok: bool,
    pub monitor_ok: bool,
}

impl Default for MsgPulsar {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgPulsar {
    pub fn new() -> Self {
        Self {
            produce_ok: true,
            consume_ok: true,
            ack_ok: true,
            schema_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.produce_ok && self.consume_ok && self.ack_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.schema_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.produce_ok || !self.consume_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.produce_ok {
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
        let c = MsgPulsar::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MsgPulsar::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MsgPulsar::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MsgPulsar::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MsgPulsar::new();
        c.produce_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MsgPulsar::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MsgPulsar::default();
        assert!(c.all_ok());
    }
}
