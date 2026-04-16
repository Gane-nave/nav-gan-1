/// GraphQL server: schema, resolver, subscription, validate, introspect
/// Phase 1051

#[derive(Debug, Clone)]
pub struct GraphqlSrv {
    pub schema_ok: bool,
    pub resolver_ok: bool,
    pub subscription_ok: bool,
    pub validate_ok: bool,
    pub introspect_ok: bool,
}

impl Default for GraphqlSrv {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphqlSrv {
    pub fn new() -> Self {
        Self {
            schema_ok: true,
            resolver_ok: true,
            subscription_ok: true,
            validate_ok: true,
            introspect_ok: true,
        }
    }

    pub fn query_ok(&self) -> bool {
        self.schema_ok && self.resolver_ok && self.validate_ok
    }

    pub fn realtime_ok(&self) -> bool {
        self.subscription_ok && self.introspect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.query_ok() && self.realtime_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.schema_ok || !self.resolver_ok
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
    fn test_query() {
        let c = GraphqlSrv::new();
        assert!(c.query_ok());
    }

    #[test]
    fn test_realtime() {
        let c = GraphqlSrv::new();
        assert!(c.realtime_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphqlSrv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = GraphqlSrv::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_schema() {
        let mut c = GraphqlSrv::new();
        c.schema_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = GraphqlSrv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
