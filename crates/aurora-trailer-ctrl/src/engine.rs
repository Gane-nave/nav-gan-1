/// trailer ctrl: connect, light, brake, sway, check
/// Phase 1283

#[derive(Debug, Clone)]
pub struct TrailerCtrl {
    pub connect_ok: bool,
    pub light_ok: bool,
    pub brake_ok: bool,
    pub sway_ok: bool,
    pub check_ok: bool,
}

impl Default for TrailerCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailerCtrl {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            light_ok: true,
            brake_ok: true,
            sway_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.light_ok && self.brake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sway_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.light_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TrailerCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TrailerCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrailerCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TrailerCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TrailerCtrl::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TrailerCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
