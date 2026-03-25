/// net multicast2: join, leave, send, receive, log
/// Phase 2267

#[derive(Debug, Clone)]
pub struct NetMulticast2 {
    pub join_ok: bool,
    pub leave_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub log_ok: bool,
}

impl Default for NetMulticast2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetMulticast2 {
    pub fn new() -> Self {
        Self {
            join_ok: true,
            leave_ok: true,
            send_ok: true,
            receive_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.join_ok && self.leave_ok && self.send_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.receive_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.join_ok || !self.leave_ok
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
        let c = NetMulticast2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetMulticast2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetMulticast2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetMulticast2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetMulticast2::new();
        c.join_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetMulticast2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
