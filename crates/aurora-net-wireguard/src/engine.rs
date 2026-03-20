/// net wireguard: configure, connect, route, monitor, log
/// Phase 2273

#[derive(Debug, Clone)]
pub struct NetWireguard {
    pub configure_ok: bool,
    pub connect_ok: bool,
    pub route_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for NetWireguard {
    fn default() -> Self {
        Self::new()
    }
}

impl NetWireguard {
    pub fn new() -> Self {
        Self {
            configure_ok: true,
            connect_ok: true,
            route_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.configure_ok && self.connect_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.configure_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.configure_ok {
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
        let c = NetWireguard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetWireguard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetWireguard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetWireguard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetWireguard::new();
        c.configure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetWireguard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
