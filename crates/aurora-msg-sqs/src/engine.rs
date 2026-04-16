/// aurora-msg-sqs: msg sqs
/// Phase 2596

#[derive(Debug, Clone)]
pub struct MsgSqs {
    pub send_ok: bool,
    pub receive_ok: bool,
    pub delete_ok: bool,
    pub dlq_ok: bool,
    pub monitor_ok: bool,
}

impl Default for MsgSqs {
    fn default() -> Self {
        Self::new()
    }
}

impl MsgSqs {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            receive_ok: true,
            delete_ok: true,
            dlq_ok: true,
            monitor_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.receive_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dlq_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.receive_ok
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
        let c = MsgSqs::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MsgSqs::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MsgSqs::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MsgSqs::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MsgSqs::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MsgSqs::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MsgSqs::default();
        assert!(c.all_ok());
    }
}
