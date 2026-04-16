/// auth policy2: create, evaluate, update, delete, log
/// Phase 2071

#[derive(Debug, Clone)]
pub struct AuthPolicy2 {
    pub create_ok: bool,
    pub evaluate_ok: bool,
    pub update_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for AuthPolicy2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPolicy2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            evaluate_ok: true,
            update_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.evaluate_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.evaluate_ok
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
        let c = AuthPolicy2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AuthPolicy2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AuthPolicy2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AuthPolicy2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AuthPolicy2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AuthPolicy2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
