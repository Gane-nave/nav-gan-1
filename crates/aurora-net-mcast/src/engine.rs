/// net mcast: join, send, recv, leave, log
/// Phase 1838

#[derive(Debug, Clone)]
pub struct NetMcast {
    pub join_ok: bool,
    pub send_ok: bool,
    pub recv_ok: bool,
    pub leave_ok: bool,
    pub log_ok: bool,
}

impl Default for NetMcast {
    fn default() -> Self {
        Self::new()
    }
}

impl NetMcast {
    pub fn new() -> Self {
        Self {
            join_ok: true,
            send_ok: true,
            recv_ok: true,
            leave_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.join_ok && self.send_ok && self.recv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.leave_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.join_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.join_ok {
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
        let c = NetMcast::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetMcast::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetMcast::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetMcast::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetMcast::new();
        c.join_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetMcast::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
