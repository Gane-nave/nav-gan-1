/// api gateway2: route, filter, transform, proxy, log
/// Phase 1847

#[derive(Debug, Clone)]
pub struct ApiGateway2 {
    pub route_ok: bool,
    pub filter_ok: bool,
    pub transform_ok: bool,
    pub proxy_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiGateway2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiGateway2 {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            filter_ok: true,
            transform_ok: true,
            proxy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.filter_ok && self.transform_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.proxy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.filter_ok
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
        let c = ApiGateway2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiGateway2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiGateway2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiGateway2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiGateway2::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiGateway2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
