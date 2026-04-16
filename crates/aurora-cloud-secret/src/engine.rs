/// aurora-cloud-secret: cloud secret
/// Phase 2553

#[derive(Debug, Clone)]
pub struct CloudSecret {
    pub store_ok: bool,
    pub retrieve_ok: bool,
    pub rotate_ok: bool,
    pub audit_ok: bool,
    pub expire_ok: bool,
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
            retrieve_ok: true,
            rotate_ok: true,
            audit_ok: true,
            expire_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.retrieve_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.expire_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.retrieve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok {
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

    #[test]
    fn test_default() {
        let c = CloudSecret::default();
        assert!(c.all_ok());
    }
}
