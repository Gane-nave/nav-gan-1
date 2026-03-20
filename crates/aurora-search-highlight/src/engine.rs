/// search highlight: mark, fragment, context, format, log
/// Phase 1884

#[derive(Debug, Clone)]
pub struct SearchHighlight {
    pub mark_ok: bool,
    pub fragment_ok: bool,
    pub context_ok: bool,
    pub format_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchHighlight {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchHighlight {
    pub fn new() -> Self {
        Self {
            mark_ok: true,
            fragment_ok: true,
            context_ok: true,
            format_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.mark_ok && self.fragment_ok && self.context_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.format_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.mark_ok || !self.fragment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.mark_ok {
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
        let c = SearchHighlight::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchHighlight::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchHighlight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchHighlight::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchHighlight::new();
        c.mark_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchHighlight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
