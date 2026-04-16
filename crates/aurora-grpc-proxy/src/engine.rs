/// aurora-grpc-proxy: grpc proxy
/// Phase 2586

#[derive(Debug, Clone)]
pub struct GrpcProxy {
    pub route_ok: bool,
    pub transform_ok: bool,
    pub auth_ok: bool,
    pub log_ok: bool,
    pub retry_ok: bool,
}

impl Default for GrpcProxy {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcProxy {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            transform_ok: true,
            auth_ok: true,
            log_ok: true,
            retry_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.transform_ok && self.auth_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.log_ok && self.retry_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.transform_ok
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
        let c = GrpcProxy::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcProxy::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcProxy::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcProxy::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcProxy::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcProxy::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcProxy::default();
        assert!(c.all_ok());
    }
}
