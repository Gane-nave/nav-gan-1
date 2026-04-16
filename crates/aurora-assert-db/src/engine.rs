/// aurora-assert-db: assert db
/// Phase 2510

#[derive(Debug, Clone)]
pub struct AssertDb {
    pub row_ok: bool,
    pub column_ok: bool,
    pub constraint_ok: bool,
    pub index_ok: bool,
    pub relation_ok: bool,
}

impl Default for AssertDb {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertDb {
    pub fn new() -> Self {
        Self {
            row_ok: true,
            column_ok: true,
            constraint_ok: true,
            index_ok: true,
            relation_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.row_ok && self.column_ok && self.constraint_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.index_ok && self.relation_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.row_ok || !self.column_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.row_ok {
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
        let c = AssertDb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertDb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertDb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertDb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertDb::new();
        c.row_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertDb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertDb::default();
        assert!(c.all_ok());
    }
}
