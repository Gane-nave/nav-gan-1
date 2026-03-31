/// aurora-mock-net: mock net
/// Phase 2490

#[derive(Debug, Clone)]
pub struct MockNet {
    pub connect_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub disconnect_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockNet {
    fn default() -> Self {
        Self::new()
    }
}

impl MockNet {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            send_ok: true,
            receive_ok: true,
            disconnect_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.send_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.disconnect_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.send_ok
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
        let c = MockNet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockNet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockNet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockNet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockNet::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockNet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockNet::default();
        assert!(c.all_ok());
    }
}
