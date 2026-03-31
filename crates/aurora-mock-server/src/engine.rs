/// aurora-mock-server: mock server
/// Phase 2482

#[derive(Debug, Clone)]
pub struct MockServer {
    pub start_ok: bool,
    pub stop_ok: bool,
    pub route_ok: bool,
    pub respond_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

impl MockServer {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            stop_ok: true,
            route_ok: true,
            respond_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.stop_ok && self.route_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.respond_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.stop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = MockServer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockServer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockServer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockServer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockServer::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockServer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockServer::default();
        assert!(c.all_ok());
    }
}
