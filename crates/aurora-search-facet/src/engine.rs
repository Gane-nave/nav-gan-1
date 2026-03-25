/// search facet: index, count, filter, aggregate, log
/// Phase 1882

#[derive(Debug, Clone)]
pub struct SearchFacet {
    pub index_ok: bool,
    pub count_ok: bool,
    pub filter_ok: bool,
    pub aggregate_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchFacet {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchFacet {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            count_ok: true,
            filter_ok: true,
            aggregate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.index_ok && self.count_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.aggregate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.index_ok || !self.count_ok
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
        let c = SearchFacet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchFacet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchFacet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchFacet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchFacet::new();
        c.index_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchFacet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
