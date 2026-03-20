/// db postgres: connect, query, insert, update, log
/// Phase 1608

#[derive(Debug, Clone)]
pub struct DbPostgres {
    pub connect_ok: bool,
    pub query_ok: bool,
    pub insert_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for DbPostgres {
    fn default() -> Self {
        Self::new()
    }
}

impl DbPostgres {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            query_ok: true,
            insert_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.query_ok && self.insert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = DbPostgres::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbPostgres::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbPostgres::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbPostgres::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbPostgres::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbPostgres::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
