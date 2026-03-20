/// ux filter: define, apply, combine, clear, log
/// Phase 1510

#[derive(Debug, Clone)]
pub struct UxFilter {
    pub define_ok: bool,
    pub apply_ok: bool,
    pub combine_ok: bool,
    pub clear_ok: bool,
    pub log_ok: bool,
}

impl Default for UxFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl UxFilter {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            apply_ok: true,
            combine_ok: true,
            clear_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.apply_ok && self.combine_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxFilter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxFilter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxFilter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxFilter::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
