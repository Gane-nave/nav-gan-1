/// relay board: open, close, toggle, interlock, status
/// Phase 1155

#[derive(Debug, Clone)]
pub struct RelayBoard {
    pub open_ok: bool,
    pub close_ok: bool,
    pub toggle_ok: bool,
    pub interlock_ok: bool,
    pub status_ok: bool,
}

impl Default for RelayBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl RelayBoard {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            toggle_ok: true,
            interlock_ok: true,
            status_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.toggle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.interlock_ok && self.status_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = RelayBoard::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RelayBoard::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RelayBoard::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RelayBoard::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RelayBoard::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RelayBoard::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
