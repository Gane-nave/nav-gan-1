/// api rest2: route, handle, validate, respond, log
/// Phase 1842

#[derive(Debug, Clone)]
pub struct ApiRest2 {
    pub route_ok: bool,
    pub handle_ok: bool,
    pub validate_ok: bool,
    pub respond_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiRest2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRest2 {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            handle_ok: true,
            validate_ok: true,
            respond_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.handle_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.respond_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.handle_ok
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
        let c = ApiRest2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiRest2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiRest2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiRest2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiRest2::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiRest2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
