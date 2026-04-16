/// net turn: allocate, relay, refresh, close, log
/// Phase 1833

#[derive(Debug, Clone)]
pub struct NetTurn {
    pub allocate_ok: bool,
    pub relay_ok: bool,
    pub refresh_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for NetTurn {
    fn default() -> Self {
        Self::new()
    }
}

impl NetTurn {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            relay_ok: true,
            refresh_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.relay_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.relay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.allocate_ok {
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
        let c = NetTurn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetTurn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetTurn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetTurn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetTurn::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetTurn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
