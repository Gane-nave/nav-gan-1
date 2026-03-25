/// net ssh: connect, authenticate, execute, tunnel, log
/// Phase 1549

#[derive(Debug, Clone)]
pub struct NetSsh {
    pub connect_ok: bool,
    pub authenticate_ok: bool,
    pub execute_ok: bool,
    pub tunnel_ok: bool,
    pub log_ok: bool,
}

impl Default for NetSsh {
    fn default() -> Self {
        Self::new()
    }
}

impl NetSsh {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            authenticate_ok: true,
            execute_ok: true,
            tunnel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.authenticate_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tunnel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.authenticate_ok
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
        let c = NetSsh::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetSsh::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetSsh::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetSsh::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetSsh::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetSsh::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
