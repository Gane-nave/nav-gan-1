/// store session: create, get, refresh, destroy, log
/// Phase 1982

#[derive(Debug, Clone)]
pub struct StoreSession {
    pub create_ok: bool,
    pub get_ok: bool,
    pub refresh_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreSession {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreSession {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            get_ok: true,
            refresh_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.get_ok && self.refresh_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.get_ok
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
        let c = StoreSession::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreSession::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreSession::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreSession::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreSession::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreSession::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
