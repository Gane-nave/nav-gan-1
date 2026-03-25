/// search spell: check, suggest, correct, learn, log
/// Phase 1885

#[derive(Debug, Clone)]
pub struct SearchSpell {
    pub check_ok: bool,
    pub suggest_ok: bool,
    pub correct_ok: bool,
    pub learn_ok: bool,
    pub log_ok: bool,
}

impl Default for SearchSpell {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchSpell {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            suggest_ok: true,
            correct_ok: true,
            learn_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.suggest_ok && self.correct_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.learn_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.suggest_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = SearchSpell::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SearchSpell::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SearchSpell::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SearchSpell::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SearchSpell::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SearchSpell::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
