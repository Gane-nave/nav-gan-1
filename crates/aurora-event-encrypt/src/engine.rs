/// event encrypt: seal, open, rotate, audit, log
/// Phase 1877

#[derive(Debug, Clone)]
pub struct EventEncrypt {
    pub seal_ok: bool,
    pub open_ok: bool,
    pub rotate_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for EventEncrypt {
    fn default() -> Self {
        Self::new()
    }
}

impl EventEncrypt {
    pub fn new() -> Self {
        Self {
            seal_ok: true,
            open_ok: true,
            rotate_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seal_ok && self.open_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seal_ok || !self.open_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seal_ok {
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
        let c = EventEncrypt::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventEncrypt::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventEncrypt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventEncrypt::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventEncrypt::new();
        c.seal_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventEncrypt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
