/// wheel align: camber, caster, toe, thrust, adjust
/// Phase 1209

#[derive(Debug, Clone)]
pub struct WheelAlign {
    pub camber_ok: bool,
    pub caster_ok: bool,
    pub toe_ok: bool,
    pub thrust_ok: bool,
    pub adjust_ok: bool,
}

impl Default for WheelAlign {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelAlign {
    pub fn new() -> Self {
        Self {
            camber_ok: true,
            caster_ok: true,
            toe_ok: true,
            thrust_ok: true,
            adjust_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.camber_ok && self.caster_ok && self.toe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.thrust_ok && self.adjust_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.camber_ok || !self.caster_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.camber_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = WheelAlign::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WheelAlign::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelAlign::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WheelAlign::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WheelAlign::new();
        c.camber_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WheelAlign::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
