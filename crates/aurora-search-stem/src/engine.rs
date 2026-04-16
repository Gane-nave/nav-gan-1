/// search stem: analyze, reduce, expand, language, log
/// Phase 1887

#[derive(Debug, Clone)]
pub struct SearchStem {
    pub analyze_ok: bool,
    pub reduce_ok: bool,
    pub expand_ok: bool,
    pub language_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchStem {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchStem {
    pub fn new() -> Self {
        Self {
            analyze_ok: true,
            reduce_ok: true,
            expand_ok: true,
            language_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.analyze_ok && self.reduce_ok && self.expand_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.language_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.analyze_ok || !self.reduce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.analyze_ok {
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
        let c = SearchStem::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchStem::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchStem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchStem::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchStem::new();
        c.analyze_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchStem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
