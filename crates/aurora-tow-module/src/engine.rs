/// tow module: hitch, signal, brake, load, check
/// Phase 1282

#[derive(Debug, Clone)]
pub struct TowModule {
    pub hitch_ok: bool,
    pub signal_ok: bool,
    pub brake_ok: bool,
    pub load_ok: bool,
    pub check_ok: bool,
}

impl Default for TowModule {
    fn default() -> Self {
        Self::new()
    }
}

impl TowModule {
    pub fn new() -> Self {
        Self {
            hitch_ok: true,
            signal_ok: true,
            brake_ok: true,
            load_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.hitch_ok && self.signal_ok && self.brake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.load_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.hitch_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hitch_ok {
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
        let c = TowModule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TowModule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TowModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TowModule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TowModule::new();
        c.hitch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TowModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
