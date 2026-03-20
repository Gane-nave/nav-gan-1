/// cargo ctrl: net, hook, light, power, check
/// Phase 1281

#[derive(Debug, Clone)]
pub struct CargoCtrl {
    pub net_ok: bool,
    pub hook_ok: bool,
    pub light_ok: bool,
    pub power_ok: bool,
    pub check_ok: bool,
}

impl Default for CargoCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl CargoCtrl {
    pub fn new() -> Self {
        Self {
            net_ok: true,
            hook_ok: true,
            light_ok: true,
            power_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.net_ok && self.hook_ok && self.light_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.power_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.net_ok || !self.hook_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.net_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CargoCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CargoCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CargoCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CargoCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CargoCtrl::new();
        c.net_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CargoCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
