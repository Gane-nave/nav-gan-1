/// index spatial: insert, query, remove, nearest, log
/// Phase 1894

#[derive(Debug, Clone)]
pub struct IndexSpatial {
    pub insert_ok: bool,
    pub query_ok: bool,
    pub remove_ok: bool,
    pub nearest_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexSpatial {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexSpatial {
    pub fn new() -> Self {
        Self {
            insert_ok: true,
            query_ok: true,
            remove_ok: true,
            nearest_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.insert_ok && self.query_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.nearest_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.insert_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.insert_ok {
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
        let c = IndexSpatial::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexSpatial::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexSpatial::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexSpatial::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexSpatial::new();
        c.insert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexSpatial::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
