/// net unix: bind, connect, send, recv, log
/// Phase 1836

#[derive(Debug, Clone)]
pub struct NetUnix {
    pub bind_ok: bool,
    pub connect_ok: bool,
    pub send_ok: bool,
    pub recv_ok: bool,
    pub log_ok: bool,
}

impl Default for NetUnix {
    fn default() -> Self {
        Self::new()
    }
}

impl NetUnix {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            connect_ok: true,
            send_ok: true,
            recv_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bind_ok && self.connect_ok && self.send_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recv_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bind_ok || !self.connect_ok
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
        let c = NetUnix::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetUnix::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetUnix::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetUnix::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetUnix::new();
        c.bind_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetUnix::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
