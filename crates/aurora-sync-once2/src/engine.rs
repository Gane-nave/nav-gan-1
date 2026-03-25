/// sync once2: call, check, reset, poison, log
/// Phase 2387

#[derive(Debug, Clone)]
pub struct SyncOnce2 {
    pub call_ok: bool,
    pub check_ok: bool,
    pub reset_ok: bool,
    pub poison_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncOnce2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncOnce2 {
    pub fn new() -> Self {
        Self {
            call_ok: true,
            check_ok: true,
            reset_ok: true,
            poison_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.call_ok && self.check_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.poison_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.call_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.call_ok {
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
        let c = SyncOnce2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncOnce2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncOnce2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncOnce2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncOnce2::new();
        c.call_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncOnce2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
