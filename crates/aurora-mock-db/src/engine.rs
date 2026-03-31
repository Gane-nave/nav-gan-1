/// aurora-mock-db: mock db
/// Phase 2484

#[derive(Debug, Clone)]
pub struct MockDb {
    pub seed_ok: bool,
    pub query_ok: bool,
    pub reset_ok: bool,
    pub transaction_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockDb {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDb {
    pub fn new() -> Self {
        Self {
            seed_ok: true,
            query_ok: true,
            reset_ok: true,
            transaction_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seed_ok && self.query_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transaction_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seed_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seed_ok {
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
        let c = MockDb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockDb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockDb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockDb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockDb::new();
        c.seed_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockDb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockDb::default();
        assert!(c.all_ok());
    }
}
