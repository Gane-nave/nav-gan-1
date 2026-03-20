/// ux search: index, query, rank, suggest, log
/// Phase 1509

#[derive(Debug, Clone)]
pub struct UxSearch {
    pub index_ok: bool,
    pub query_ok: bool,
    pub rank_ok: bool,
    pub suggest_ok: bool,
    pub log_ok: bool,
}

impl Default for UxSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl UxSearch {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            query_ok: true,
            rank_ok: true,
            suggest_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.index_ok && self.query_ok && self.rank_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.suggest_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.index_ok || !self.query_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.index_ok {
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
        let c = UxSearch::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxSearch::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxSearch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxSearch::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxSearch::new();
        c.index_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxSearch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
