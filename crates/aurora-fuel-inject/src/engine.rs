/// fuel inject: spray, pulse, trim, purge, check
/// Phase 1232

#[derive(Debug, Clone)]
pub struct FuelInject {
    pub spray_ok: bool,
    pub pulse_ok: bool,
    pub trim_ok: bool,
    pub purge_ok: bool,
    pub check_ok: bool,
}

impl Default for FuelInject {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelInject {
    pub fn new() -> Self {
        Self {
            spray_ok: true,
            pulse_ok: true,
            trim_ok: true,
            purge_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spray_ok && self.pulse_ok && self.trim_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.purge_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spray_ok || !self.pulse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spray_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FuelInject::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuelInject::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelInject::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuelInject::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuelInject::new();
        c.spray_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuelInject::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
