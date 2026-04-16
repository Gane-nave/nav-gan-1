/// aurora-ui-flex: ui flex
/// Phase 2411

#[derive(Debug, Clone)]
pub struct UiFlex {
    pub direction_ok: bool,
    pub wrap_ok: bool,
    pub align_ok: bool,
    pub justify_ok: bool,
    pub gap_ok: bool,
}

impl Default for UiFlex {
    fn default() -> Self {
        Self::new()
    }
}

impl UiFlex {
    pub fn new() -> Self {
        Self {
            direction_ok: true,
            wrap_ok: true,
            align_ok: true,
            justify_ok: true,
            gap_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.direction_ok && self.wrap_ok && self.align_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.justify_ok && self.gap_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.direction_ok || !self.wrap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.direction_ok {
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
        let c = UiFlex::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiFlex::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiFlex::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiFlex::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiFlex::new();
        c.direction_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiFlex::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiFlex::default();
        assert!(c.all_ok());
    }
}
