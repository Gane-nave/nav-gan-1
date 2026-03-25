/// runtime channel: send, recv, close, select, log
/// Phase 1799

#[derive(Debug, Clone)]
pub struct RuntimeChannel {
    pub send_ok: bool,
    pub recv_ok: bool,
    pub close_ok: bool,
    pub select_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeChannel {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            recv_ok: true,
            close_ok: true,
            select_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.recv_ok && self.close_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.select_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.recv_ok
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
        let c = RuntimeChannel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeChannel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeChannel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeChannel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeChannel::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeChannel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
