/// gRPC server: service, method, stream, intercept, reflect
/// Phase 1050

#[derive(Debug, Clone)]
pub struct GrpcServer {
    pub service_ok: bool,
    pub method_ok: bool,
    pub stream_ok: bool,
    pub intercept_ok: bool,
    pub reflect_ok: bool,
}

impl Default for GrpcServer {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcServer {
    pub fn new() -> Self {
        Self {
            service_ok: true,
            method_ok: true,
            stream_ok: true,
            intercept_ok: true,
            reflect_ok: true,
        }
    }

    pub fn serving_ok(&self) -> bool {
        self.service_ok && self.method_ok && self.stream_ok
    }

    pub fn middleware_ok(&self) -> bool {
        self.intercept_ok && self.reflect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.serving_ok() && self.middleware_ok()
    }

    pub fn needs_restart(&self) -> bool {
        !self.service_ok || !self.method_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.service_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serving() {
        let c = GrpcServer::new();
        assert!(c.serving_ok());
    }

    #[test]
    fn test_middleware() {
        let c = GrpcServer::new();
        assert!(c.middleware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcServer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_restart() {
        let c = GrpcServer::new();
        assert!(!c.needs_restart());
    }

    #[test]
    fn test_service() {
        let mut c = GrpcServer::new();
        c.service_ok = false;
        assert!(c.needs_restart());
    }

    #[test]
    fn test_health() {
        let c = GrpcServer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
