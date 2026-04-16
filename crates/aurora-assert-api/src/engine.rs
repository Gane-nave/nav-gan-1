/// aurora-assert-api: assert api
/// Phase 2509

#[derive(Debug, Clone)]
pub struct AssertApi {
    pub status_ok: bool,
    pub header_ok: bool,
    pub body_ok: bool,
    pub latency_ok: bool,
    pub schema_ok: bool,
}

impl Default for AssertApi {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertApi {
    pub fn new() -> Self {
        Self {
            status_ok: true,
            header_ok: true,
            body_ok: true,
            latency_ok: true,
            schema_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.status_ok && self.header_ok && self.body_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.latency_ok && self.schema_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.status_ok || !self.header_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.status_ok {
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
        let c = AssertApi::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertApi::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertApi::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertApi::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertApi::new();
        c.status_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertApi::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertApi::default();
        assert!(c.all_ok());
    }
}
