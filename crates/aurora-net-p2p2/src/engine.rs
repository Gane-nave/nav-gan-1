/// net p2p2: discover, connect, exchange, verify, log
/// Phase 2266

#[derive(Debug, Clone)]
pub struct NetP2p2 {
    pub discover_ok: bool,
    pub connect_ok: bool,
    pub exchange_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for NetP2p2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetP2p2 {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            connect_ok: true,
            exchange_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.connect_ok && self.exchange_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = NetP2p2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetP2p2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetP2p2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetP2p2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetP2p2::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetP2p2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
