/// api grpc3: define, serve, intercept, stream, log
/// Phase 1844

#[derive(Debug, Clone)]
pub struct ApiGrpc3 {
    pub define_ok: bool,
    pub serve_ok: bool,
    pub intercept_ok: bool,
    pub stream_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiGrpc3 {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiGrpc3 {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            serve_ok: true,
            intercept_ok: true,
            stream_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.serve_ok && self.intercept_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.serve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = ApiGrpc3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiGrpc3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiGrpc3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiGrpc3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiGrpc3::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiGrpc3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
