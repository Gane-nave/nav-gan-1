/// aurora-grpc-client: grpc client
/// Phase 2585

#[derive(Debug, Clone)]
pub struct GrpcClient {
    pub connect_ok: bool,
    pub call_ok: bool,
    pub retry_ok: bool,
    pub timeout_ok: bool,
    pub lb_ok: bool,
}

impl Default for GrpcClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcClient {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            call_ok: true,
            retry_ok: true,
            timeout_ok: true,
            lb_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.call_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.timeout_ok && self.lb_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.call_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = GrpcClient::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcClient::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcClient::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcClient::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcClient::default();
        assert!(c.all_ok());
    }
}
