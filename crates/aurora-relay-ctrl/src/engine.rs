/// relay ctrl: energize, hold, release, monitor, log
/// Phase 1368

#[derive(Debug, Clone)]
pub struct RelayCtrl {
    pub energize_ok: bool,
    pub hold_ok: bool,
    pub release_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for RelayCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl RelayCtrl {
    pub fn new() -> Self {
        Self {
            energize_ok: true,
            hold_ok: true,
            release_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.energize_ok && self.hold_ok && self.release_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.energize_ok || !self.hold_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.energize_ok {
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
        let c = RelayCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RelayCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RelayCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RelayCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RelayCtrl::new();
        c.energize_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RelayCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
