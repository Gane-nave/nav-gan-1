/// alternator: generate, regulate, rectify, cool, check
/// Phase 1225

#[derive(Debug, Clone)]
pub struct Alternator {
    pub generate_ok: bool,
    pub regulate_ok: bool,
    pub rectify_ok: bool,
    pub cool_ok: bool,
    pub check_ok: bool,
}

impl Default for Alternator {
    fn default() -> Self {
        Self::new()
    }
}

impl Alternator {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            regulate_ok: true,
            rectify_ok: true,
            cool_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.regulate_ok && self.rectify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cool_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.regulate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Alternator::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Alternator::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Alternator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Alternator::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Alternator::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Alternator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
