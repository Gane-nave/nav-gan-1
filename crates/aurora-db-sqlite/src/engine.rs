/// db sqlite: open, query, insert, update, log
/// Phase 1610

#[derive(Debug, Clone)]
pub struct DbSqlite {
    pub open_ok: bool,
    pub query_ok: bool,
    pub insert_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for DbSqlite {
    fn default() -> Self {
        Self::new()
    }
}

impl DbSqlite {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            query_ok: true,
            insert_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.query_ok && self.insert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.query_ok
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
        let c = DbSqlite::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbSqlite::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbSqlite::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbSqlite::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbSqlite::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbSqlite::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
