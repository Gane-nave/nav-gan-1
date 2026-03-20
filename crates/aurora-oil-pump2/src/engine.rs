/// oil pump: draw, pressurize, deliver, relief, check
/// Phase 1251

#[derive(Debug, Clone)]
pub struct OilPump2 {
    pub draw_ok: bool,
    pub pressurize_ok: bool,
    pub deliver_ok: bool,
    pub relief_ok: bool,
    pub check_ok: bool,
}

impl Default for OilPump2 {
    fn default() -> Self {
        Self::new()
    }
}

impl OilPump2 {
    pub fn new() -> Self {
        Self {
            draw_ok: true,
            pressurize_ok: true,
            deliver_ok: true,
            relief_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.draw_ok && self.pressurize_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.relief_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.draw_ok || !self.pressurize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.draw_ok {
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
        let c = OilPump2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OilPump2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilPump2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OilPump2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OilPump2::new();
        c.draw_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OilPump2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
