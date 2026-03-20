/// api csrf: generate, validate, rotate, embed, log
/// Phase 1857

#[derive(Debug, Clone)]
pub struct ApiCsrf {
    pub generate_ok: bool,
    pub validate_ok: bool,
    pub rotate_ok: bool,
    pub embed_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiCsrf {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCsrf {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            validate_ok: true,
            rotate_ok: true,
            embed_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.validate_ok && self.rotate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.embed_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = ApiCsrf::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiCsrf::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiCsrf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiCsrf::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiCsrf::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiCsrf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
