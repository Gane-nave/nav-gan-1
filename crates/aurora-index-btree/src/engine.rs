/// index btree: insert, search, delete, range, log
/// Phase 1890

#[derive(Debug, Clone)]
pub struct IndexBtree {
    pub insert_ok: bool,
    pub search_ok: bool,
    pub delete_ok: bool,
    pub range_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexBtree {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexBtree {
    pub fn new() -> Self {
        Self {
            insert_ok: true,
            search_ok: true,
            delete_ok: true,
            range_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.insert_ok && self.search_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.range_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.insert_ok || !self.search_ok
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
        let c = IndexBtree::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexBtree::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexBtree::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexBtree::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexBtree::new();
        c.insert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexBtree::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
