/// net broadcast2: send, receive, filter, dedupe, log
/// Phase 2268

#[derive(Debug, Clone)]
pub struct NetBroadcast2 {
    pub send_ok: bool,
    pub receive_ok: bool,
    pub filter_ok: bool,
    pub dedupe_ok: bool,
    pub log_ok: bool,
}

impl Default for NetBroadcast2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetBroadcast2 {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            receive_ok: true,
            filter_ok: true,
            dedupe_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.receive_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dedupe_ok && self.log_ok
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
        let c = NetBroadcast2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetBroadcast2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetBroadcast2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetBroadcast2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetBroadcast2::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetBroadcast2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
