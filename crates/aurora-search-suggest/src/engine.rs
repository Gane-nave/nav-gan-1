/// search suggest: index, complete, correct, rank, log
/// Phase 1883

#[derive(Debug, Clone)]
pub struct SearchSuggest {
    pub index_ok: bool,
    pub complete_ok: bool,
    pub correct_ok: bool,
    pub rank_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchSuggest {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchSuggest {
    pub fn new() -> Self {
        Self {
            index_ok: true,
            complete_ok: true,
            correct_ok: true,
            rank_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.index_ok && self.complete_ok && self.correct_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rank_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.index_ok || !self.complete_ok
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
        let c = SearchSuggest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchSuggest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchSuggest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchSuggest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchSuggest::new();
        c.index_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchSuggest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
