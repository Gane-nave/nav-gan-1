/// Wireless charging pad: Qi, alignment, temp, foreign obj
/// Phase 903

#[derive(Debug, Clone)]
pub struct WirelessPad {
    pub qi_ok: bool,
    pub alignment_ok: bool,
    pub temp_ok: bool,
    pub fod_ok: bool,
    pub power_ok: bool,
}

impl Default for WirelessPad {
    fn default() -> Self {
        Self::new()
    }
}

impl WirelessPad {
    pub fn new() -> Self {
        Self {
            qi_ok: true,
            alignment_ok: true,
            temp_ok: true,
            fod_ok: true,
            power_ok: true,
        }
    }

    pub fn charging_ok(&self) -> bool {
        self.qi_ok && self.alignment_ok && self.power_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.temp_ok && self.fod_ok
    }

    pub fn all_ok(&self) -> bool {
        self.charging_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.qi_ok || !self.power_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.qi_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charging() {
        let c = WirelessPad::new();
        assert!(c.charging_ok());
    }

    #[test]
    fn test_safety() {
        let c = WirelessPad::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WirelessPad::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WirelessPad::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_qi() {
        let mut c = WirelessPad::new();
        c.qi_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WirelessPad::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
