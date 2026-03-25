/// proto dns2: query, resolve, cache, update, log
/// Phase 2010

#[derive(Debug, Clone)]
pub struct ProtoDns2 {
    pub query_ok: bool,
    pub resolve_ok: bool,
    pub cache_ok: bool,
    pub update_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoDns2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoDns2 {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            resolve_ok: true,
            cache_ok: true,
            update_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.query_ok && self.resolve_ok && self.cache_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.update_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.query_ok || !self.resolve_ok
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
        let c = ProtoDns2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoDns2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoDns2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoDns2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoDns2::new();
        c.query_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoDns2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
