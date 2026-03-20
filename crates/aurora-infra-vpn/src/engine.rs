/// infra vpn: create, connect, disconnect, monitor, log
/// Phase 2138

#[derive(Debug, Clone)]
pub struct InfraVpn {
    pub create_ok: bool,
    pub connect_ok: bool,
    pub disconnect_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraVpn {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraVpn {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            connect_ok: true,
            disconnect_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.connect_ok && self.disconnect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = InfraVpn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraVpn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraVpn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraVpn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraVpn::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraVpn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
