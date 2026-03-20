/// valid currency: parse, format, convert, validate, log
/// Phase 2093

#[derive(Debug, Clone)]
pub struct ValidCurrency {
    pub parse_ok: bool,
    pub format_ok: bool,
    pub convert_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidCurrency {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidCurrency {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            format_ok: true,
            convert_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.format_ok && self.convert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.format_ok
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
        let c = ValidCurrency::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidCurrency::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidCurrency::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidCurrency::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidCurrency::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidCurrency::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
