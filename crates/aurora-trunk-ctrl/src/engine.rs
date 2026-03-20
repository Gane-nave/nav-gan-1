/// trunk ctrl: unlock, open, close, lock, kick
/// Phase 1190

#[derive(Debug, Clone)]
pub struct TrunkCtrl {
    pub unlock_ok: bool,
    pub open_ok: bool,
    pub close_ok: bool,
    pub lock_ok: bool,
    pub kick_ok: bool,
}

impl Default for TrunkCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkCtrl {
    pub fn new() -> Self {
        Self {
            unlock_ok: true,
            open_ok: true,
            close_ok: true,
            lock_ok: true,
            kick_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.unlock_ok && self.open_ok && self.close_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.lock_ok && self.kick_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.unlock_ok || !self.open_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.unlock_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TrunkCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TrunkCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrunkCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TrunkCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TrunkCtrl::new();
        c.unlock_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TrunkCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
