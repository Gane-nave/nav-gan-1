/// search synonym: add, expand, remove, list, log
/// Phase 1886

#[derive(Debug, Clone)]
pub struct SearchSynonym {
    pub add_ok: bool,
    pub expand_ok: bool,
    pub remove_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchSynonym {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchSynonym {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            expand_ok: true,
            remove_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.expand_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.expand_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = SearchSynonym::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchSynonym::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchSynonym::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchSynonym::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchSynonym::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchSynonym::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
