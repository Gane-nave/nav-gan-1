/// Climate control: HVAC, blend door, recirculation
/// Phase 540

#[derive(Debug, Clone)]
pub struct ClimateControl {
    pub cabin_temp_c: f64,
    pub target_temp_c: f64,
    pub blower_ok: bool,
    pub compressor_ok: bool,
    pub blend_door_ok: bool,
}

impl Default for ClimateControl {
    fn default() -> Self {
        Self::new()
    }
}

impl ClimateControl {
    pub fn new() -> Self {
        Self {
            cabin_temp_c: 22.0,
            target_temp_c: 22.0,
            blower_ok: true,
            compressor_ok: true,
            blend_door_ok: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.cabin_temp_c - self.target_temp_c).abs() < 2.0
    }

    pub fn system_ok(&self) -> bool {
        self.blower_ok && self.compressor_ok && self.blend_door_ok
    }

    pub fn all_ok(&self) -> bool {
        self.at_target() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.compressor_ok || !self.blower_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compressor_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let c = ClimateControl::new();
        assert!(c.at_target());
    }

    #[test]
    fn test_system() {
        let c = ClimateControl::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClimateControl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ClimateControl::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_compressor() {
        let mut c = ClimateControl::new();
        c.compressor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ClimateControl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
