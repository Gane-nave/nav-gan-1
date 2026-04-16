/// api versioning: route, migrate, deprecate, redirect, log
/// Phase 1850

#[derive(Debug, Clone)]
pub struct ApiVersioning {
    pub route_ok: bool,
    pub migrate_ok: bool,
    pub deprecate_ok: bool,
    pub redirect_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiVersioning {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiVersioning {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            migrate_ok: true,
            deprecate_ok: true,
            redirect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.migrate_ok && self.deprecate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.redirect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.migrate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = ApiVersioning::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiVersioning::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiVersioning::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiVersioning::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiVersioning::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiVersioning::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
