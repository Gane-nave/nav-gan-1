/// aurora-mock-api: mock api
/// Phase 2483

#[derive(Debug, Clone)]
pub struct MockApi {
    pub define_ok: bool,
    pub route_ok: bool,
    pub respond_ok: bool,
    pub delay_ok: bool,
    pub error_ok: bool,
}

impl Default for MockApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MockApi {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            route_ok: true,
            respond_ok: true,
            delay_ok: true,
            error_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.route_ok && self.respond_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delay_ok && self.error_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.route_ok
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
        let c = MockApi::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockApi::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockApi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockApi::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockApi::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockApi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockApi::default();
        assert!(c.all_ok());
    }
}
