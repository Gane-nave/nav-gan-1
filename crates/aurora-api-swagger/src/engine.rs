/// api swagger: parse, validate, generate, serve, log
/// Phase 1846

#[derive(Debug, Clone)]
pub struct ApiSwagger {
    pub parse_ok: bool,
    pub validate_ok: bool,
    pub generate_ok: bool,
    pub serve_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiSwagger {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiSwagger {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            validate_ok: true,
            generate_ok: true,
            serve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.validate_ok && self.generate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.serve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.validate_ok
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
        let c = ApiSwagger::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiSwagger::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiSwagger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiSwagger::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiSwagger::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiSwagger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
