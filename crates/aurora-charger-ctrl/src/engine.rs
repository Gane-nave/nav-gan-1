/// charger ctrl: connect, negotiate, deliver, monitor, disconnect
/// Phase 1142

#[derive(Debug, Clone)]
pub struct ChargerCtrl {
    pub connect_ok: bool,
    pub negotiate_ok: bool,
    pub deliver_ok: bool,
    pub monitor_ok: bool,
    pub disconnect_ok: bool,
}

impl Default for ChargerCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargerCtrl {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            negotiate_ok: true,
            deliver_ok: true,
            monitor_ok: true,
            disconnect_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.negotiate_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.disconnect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.negotiate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ChargerCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChargerCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargerCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChargerCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChargerCtrl::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChargerCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
