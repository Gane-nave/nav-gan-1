/// db duckdb: open, query, insert, export, log
/// Phase 1624

#[derive(Debug, Clone)]
pub struct DbDuckdb {
    pub open_ok: bool,
    pub query_ok: bool,
    pub insert_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for DbDuckdb {
    fn default() -> Self {
        Self::new()
    }
}

impl DbDuckdb {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            query_ok: true,
            insert_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.query_ok && self.insert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
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
        let c = DbDuckdb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbDuckdb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbDuckdb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbDuckdb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbDuckdb::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbDuckdb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
