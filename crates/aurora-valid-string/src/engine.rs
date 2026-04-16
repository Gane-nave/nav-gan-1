/// valid string: length, pattern, charset, sanitize, log
/// Phase 2076

#[derive(Debug, Clone)]
pub struct ValidString {
    pub length_ok: bool,
    pub pattern_ok: bool,
    pub charset_ok: bool,
    pub sanitize_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidString {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidString {
    pub fn new() -> Self {
        Self {
            length_ok: true,
            pattern_ok: true,
            charset_ok: true,
            sanitize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.length_ok && self.pattern_ok && self.charset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sanitize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.length_ok || !self.pattern_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.length_ok {
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
        let c = ValidString::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidString::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidString::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidString::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidString::new();
        c.length_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidString::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
