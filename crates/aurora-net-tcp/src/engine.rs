/// net tcp: listen, accept, read, write, log
/// Phase 1536

#[derive(Debug, Clone)]
pub struct NetTcp {
    pub listen_ok: bool,
    pub accept_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub log_ok: bool,
}

impl Default for NetTcp {
    fn default() -> Self {
        Self::new()
    }
}

impl NetTcp {
    pub fn new() -> Self {
        Self {
            listen_ok: true,
            accept_ok: true,
            read_ok: true,
            write_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.listen_ok && self.accept_ok && self.read_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.write_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.listen_ok || !self.accept_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.listen_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = NetTcp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetTcp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetTcp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetTcp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetTcp::new();
        c.listen_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetTcp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
