/// aurora-fuzz-http: fuzz http
/// Phase 2492

#[derive(Debug, Clone)]
pub struct FuzzHttp {
    pub request_ok: bool,
    pub response_ok: bool,
    pub header_ok: bool,
    pub body_ok: bool,
    pub inject_ok: bool,
}

impl Default for FuzzHttp {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzHttp {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            response_ok: true,
            header_ok: true,
            body_ok: true,
            inject_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.response_ok && self.header_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.body_ok && self.inject_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.response_ok
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
        let c = FuzzHttp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuzzHttp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuzzHttp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuzzHttp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuzzHttp::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuzzHttp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = FuzzHttp::default();
        assert!(c.all_ok());
    }
}
