/// net udp2: bind, send, receive, broadcast, log
/// Phase 2257

#[derive(Debug, Clone)]
pub struct NetUdp2 {
    pub bind_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub broadcast_ok: bool,
    pub log_ok: bool,
}

impl Default for NetUdp2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetUdp2 {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            send_ok: true,
            receive_ok: true,
            broadcast_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bind_ok && self.send_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.broadcast_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bind_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bind_ok {
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
        let c = NetUdp2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetUdp2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetUdp2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetUdp2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetUdp2::new();
        c.bind_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetUdp2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
