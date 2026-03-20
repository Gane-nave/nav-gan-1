/// vapor canister: store, purge, vent, load, check
/// Phase 1261

#[derive(Debug, Clone)]
pub struct VaporCanist {
    pub store_ok: bool,
    pub purge_ok: bool,
    pub vent_ok: bool,
    pub load_ok: bool,
    pub check_ok: bool,
}

impl Default for VaporCanist {
    fn default() -> Self {
        Self::new()
    }
}

impl VaporCanist {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            purge_ok: true,
            vent_ok: true,
            load_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.purge_ok && self.vent_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.load_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.purge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VaporCanist::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VaporCanist::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VaporCanist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VaporCanist::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VaporCanist::new();
        c.store_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VaporCanist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
