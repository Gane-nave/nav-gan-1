/// net http: request, respond, route, middleware, log
/// Phase 1538

#[derive(Debug, Clone)]
pub struct NetHttp {
    pub request_ok: bool,
    pub respond_ok: bool,
    pub route_ok: bool,
    pub middleware_ok: bool,
    pub log_ok: bool,
}

impl Default for NetHttp {
    fn default() -> Self {
        Self::new()
    }
}

impl NetHttp {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            respond_ok: true,
            route_ok: true,
            middleware_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.respond_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.middleware_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.respond_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = NetHttp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetHttp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetHttp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetHttp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetHttp::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetHttp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
