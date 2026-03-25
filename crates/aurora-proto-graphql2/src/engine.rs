/// proto graphql2: query, mutate, subscribe, introspect, log
/// Phase 2021

#[derive(Debug, Clone)]
pub struct ProtoGraphql2 {
    pub query_ok: bool,
    pub mutate_ok: bool,
    pub subscribe_ok: bool,
    pub introspect_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoGraphql2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoGraphql2 {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            mutate_ok: true,
            subscribe_ok: true,
            introspect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.query_ok && self.mutate_ok && self.subscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.introspect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.query_ok || !self.mutate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok {
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
        let c = ProtoGraphql2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoGraphql2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoGraphql2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoGraphql2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoGraphql2::new();
        c.query_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoGraphql2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
