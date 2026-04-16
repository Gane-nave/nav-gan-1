/// sync broadcast3: send, subscribe, recv, close, log
/// Phase 2397

#[derive(Debug, Clone)]
pub struct SyncBroadcast3 {
    pub send_ok: bool,
    pub subscribe_ok: bool,
    pub recv_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncBroadcast3 {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncBroadcast3 {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            subscribe_ok: true,
            recv_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.subscribe_ok && self.recv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.subscribe_ok
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
        let c = SyncBroadcast3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncBroadcast3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncBroadcast3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncBroadcast3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncBroadcast3::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncBroadcast3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
