/// aurora-grpc-server: grpc server
/// Phase 2584

#[derive(Debug, Clone)]
pub struct GrpcServer {
    pub listen_ok: bool,
    pub route_ok: bool,
    pub auth_ok: bool,
    pub tls_ok: bool,
    pub health_ok: bool,
}

impl Default for GrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcServer {
    pub fn new() -> Self {
        Self {
            listen_ok: true,
            route_ok: true,
            auth_ok: true,
            tls_ok: true,
            health_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.listen_ok && self.route_ok && self.auth_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tls_ok && self.health_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.listen_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.listen_ok {
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
        let c = GrpcServer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcServer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcServer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcServer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcServer::new();
        c.listen_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcServer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcServer::default();
        assert!(c.all_ok());
    }
}
