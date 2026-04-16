/// flexray: sync, static, dynamic, guard, check
/// Phase 1271

#[derive(Debug, Clone)]
pub struct Flexray {
    pub sync_ok: bool,
    pub static_ok: bool,
    pub dynamic_ok: bool,
    pub guard_ok: bool,
    pub check_ok: bool,
}

impl Default for Flexray {
    fn default() -> Self {
        Self::new()
    }
}

impl Flexray {
    pub fn new() -> Self {
        Self {
            sync_ok: true,
            static_ok: true,
            dynamic_ok: true,
            guard_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sync_ok && self.static_ok && self.dynamic_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.guard_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sync_ok || !self.static_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sync_ok {
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
        let c = Flexray::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Flexray::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Flexray::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Flexray::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Flexray::new();
        c.sync_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Flexray::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
