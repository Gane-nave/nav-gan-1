/// search token: analyze, filter, normalize, split, log
/// Phase 1888

#[derive(Debug, Clone)]
pub struct SearchToken {
    pub analyze_ok: bool,
    pub filter_ok: bool,
    pub normalize_ok: bool,
    pub split_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchToken {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchToken {
    pub fn new() -> Self {
        Self {
            analyze_ok: true,
            filter_ok: true,
            normalize_ok: true,
            split_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.analyze_ok && self.filter_ok && self.normalize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.split_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.analyze_ok || !self.filter_ok
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
        let c = SearchToken::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchToken::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchToken::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchToken::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchToken::new();
        c.analyze_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchToken::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
