/// cloud secret: store, rotate, access, audit, log
/// Phase 1456

#[derive(Debug, Clone)]
pub struct CloudSecret {
    pub store_ok: bool,
    pub rotate_ok: bool,
    pub access_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudSecret {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudSecret {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            rotate_ok: true,
            access_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.rotate_ok && self.access_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.rotate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudSecret::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudSecret::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudSecret::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudSecret::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudSecret::new();
        c.store_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudSecret::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
