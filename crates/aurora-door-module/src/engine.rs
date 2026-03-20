/// door module: lock, unlock, window, mirror, check
/// Phase 1275

#[derive(Debug, Clone)]
pub struct DoorModule {
    pub lock_ok: bool,
    pub unlock_ok: bool,
    pub window_ok: bool,
    pub mirror_ok: bool,
    pub check_ok: bool,
}

impl Default for DoorModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DoorModule {
    pub fn new() -> Self {
        Self {
            lock_ok: true,
            unlock_ok: true,
            window_ok: true,
            mirror_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lock_ok && self.unlock_ok && self.window_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.mirror_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lock_ok || !self.unlock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lock_ok {
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
        let c = DoorModule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DoorModule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DoorModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DoorModule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DoorModule::new();
        c.lock_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DoorModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
