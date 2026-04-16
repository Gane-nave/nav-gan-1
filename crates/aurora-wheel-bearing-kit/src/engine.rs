/// Wheel bearing kit: bearing, seal, nut, cotter pin
/// Phase 814

#[derive(Debug, Clone)]
pub struct WheelBearingKit {
    pub bearing_ok: bool,
    pub seal_ok: bool,
    pub nut_ok: bool,
    pub pin_ok: bool,
    pub preload_ok: bool,
}

impl Default for WheelBearingKit {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelBearingKit {
    pub fn new() -> Self {
        Self {
            bearing_ok: true,
            seal_ok: true,
            nut_ok: true,
            pin_ok: true,
            preload_ok: true,
        }
    }

    pub fn assembly_ok(&self) -> bool {
        self.bearing_ok && self.seal_ok
    }

    pub fn fastening_ok(&self) -> bool {
        self.nut_ok && self.pin_ok && self.preload_ok
    }

    pub fn all_ok(&self) -> bool {
        self.assembly_ok() && self.fastening_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly() {
        let c = WheelBearingKit::new();
        assert!(c.assembly_ok());
    }

    #[test]
    fn test_fastening() {
        let c = WheelBearingKit::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelBearingKit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WheelBearingKit::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing() {
        let mut c = WheelBearingKit::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WheelBearingKit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
