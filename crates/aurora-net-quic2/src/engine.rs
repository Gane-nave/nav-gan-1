/// net quic2: connect, stream, send, receive, log
/// Phase 2258

#[derive(Debug, Clone)]
pub struct NetQuic2 {
    pub connect_ok: bool,
    pub stream_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub log_ok: bool,
}

impl Default for NetQuic2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetQuic2 {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            stream_ok: true,
            send_ok: true,
            receive_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.stream_ok && self.send_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.receive_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.stream_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = NetQuic2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetQuic2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetQuic2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetQuic2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetQuic2::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetQuic2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
