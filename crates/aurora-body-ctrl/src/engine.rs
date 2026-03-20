/// body ctrl: light, lock, wiper, horn, check
/// Phase 1274

#[derive(Debug, Clone)]
pub struct BodyCtrl {
    pub light_ok: bool,
    pub lock_ok: bool,
    pub wiper_ok: bool,
    pub horn_ok: bool,
    pub check_ok: bool,
}

impl Default for BodyCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyCtrl {
    pub fn new() -> Self {
        Self {
            light_ok: true,
            lock_ok: true,
            wiper_ok: true,
            horn_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.light_ok && self.lock_ok && self.wiper_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.horn_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.light_ok || !self.lock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.light_ok {
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
        let c = BodyCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BodyCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BodyCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BodyCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BodyCtrl::new();
        c.light_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BodyCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
