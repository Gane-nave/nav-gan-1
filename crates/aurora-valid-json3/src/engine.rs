/// valid json3: parse, schema, validate, transform, log
/// Phase 2085

#[derive(Debug, Clone)]
pub struct ValidJson3 {
    pub parse_ok: bool,
    pub schema_ok: bool,
    pub validate_ok: bool,
    pub transform_ok: bool,
    pub log_ok: bool,
}

impl Default for ValidJson3 {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidJson3 {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            schema_ok: true,
            validate_ok: true,
            transform_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.schema_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transform_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.schema_ok
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
        let c = ValidJson3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValidJson3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValidJson3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValidJson3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValidJson3::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValidJson3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
