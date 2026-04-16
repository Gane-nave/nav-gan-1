/// api sort: parse, apply, validate, combine, log
/// Phase 1853

#[derive(Debug, Clone)]
pub struct ApiSort {
    pub parse_ok: bool,
    pub apply_ok: bool,
    pub validate_ok: bool,
    pub combine_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiSort {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiSort {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            apply_ok: true,
            validate_ok: true,
            combine_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.apply_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.combine_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = ApiSort::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiSort::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiSort::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiSort::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiSort::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiSort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
