/// api graphql2: schema, resolve, subscribe, execute, log
/// Phase 1843

#[derive(Debug, Clone)]
pub struct ApiGraphql2 {
    pub schema_ok: bool,
    pub resolve_ok: bool,
    pub subscribe_ok: bool,
    pub execute_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiGraphql2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiGraphql2 {
    pub fn new() -> Self {
        Self {
            schema_ok: true,
            resolve_ok: true,
            subscribe_ok: true,
            execute_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schema_ok && self.resolve_ok && self.subscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.execute_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schema_ok || !self.resolve_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schema_ok {
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
        let c = ApiGraphql2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiGraphql2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiGraphql2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiGraphql2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiGraphql2::new();
        c.schema_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiGraphql2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
