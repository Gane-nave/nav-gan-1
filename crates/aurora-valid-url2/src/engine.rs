/// valid url2: parse, normalize, validate, resolve, log
/// Phase 2080

#[derive(Debug, Clone)]
pub struct ValidUrl2 {
    pub parse_ok: bool,
    pub normalize_ok: bool,
    pub validate_ok: bool,
    pub resolve_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidUrl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidUrl2 {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            normalize_ok: true,
            validate_ok: true,
            resolve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.normalize_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resolve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.normalize_ok
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
        let c = ValidUrl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidUrl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidUrl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidUrl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidUrl2::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidUrl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
