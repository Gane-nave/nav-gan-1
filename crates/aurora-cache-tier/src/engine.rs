/// cache tier: promote, demote, lookup, migrate, log
/// Phase 1940

#[derive(Debug, Clone)]
pub struct CacheTier {
    pub promote_ok: bool,
    pub demote_ok: bool,
    pub lookup_ok: bool,
    pub migrate_ok: bool,
    pub log_ok: bool,
}

impl Default for CacheTier {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheTier {
    pub fn new() -> Self {
        Self {
            promote_ok: true,
            demote_ok: true,
            lookup_ok: true,
            migrate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.promote_ok && self.demote_ok && self.lookup_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.migrate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.promote_ok || !self.demote_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.promote_ok {
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
        let c = CacheTier::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CacheTier::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CacheTier::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CacheTier::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CacheTier::new();
        c.promote_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CacheTier::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
