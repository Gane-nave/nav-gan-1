/// trans heartbeat2: send, receive, timeout, recover, log
/// Phase 2285

#[derive(Debug, Clone)]
pub struct TransHeartbeat2 {
    pub send_ok: bool,
    pub receive_ok: bool,
    pub timeout_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for TransHeartbeat2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransHeartbeat2 {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            receive_ok: true,
            timeout_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.receive_ok && self.timeout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
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
        let c = TransHeartbeat2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransHeartbeat2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransHeartbeat2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransHeartbeat2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransHeartbeat2::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransHeartbeat2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
