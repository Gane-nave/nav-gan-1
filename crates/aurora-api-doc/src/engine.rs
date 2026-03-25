/// api doc: generate, serve, validate, export, log
/// Phase 1859

#[derive(Debug, Clone)]
pub struct ApiDoc {
    pub generate_ok: bool,
    pub serve_ok: bool,
    pub validate_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiDoc {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiDoc {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            serve_ok: true,
            validate_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.serve_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.serve_ok
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
        let c = ApiDoc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiDoc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiDoc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiDoc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiDoc::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiDoc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
