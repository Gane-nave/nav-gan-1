/// db elastic: connect, search, index, delete, log
/// Phase 1619

#[derive(Debug, Clone)]
pub struct DbElastic {
    pub connect_ok: bool,
    pub search_ok: bool,
    pub index_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for DbElastic {
    fn default() -> Self {
        Self::new()
    }
}

impl DbElastic {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            search_ok: true,
            index_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.search_ok && self.index_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.search_ok
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
        let c = DbElastic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbElastic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbElastic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbElastic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbElastic::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbElastic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
