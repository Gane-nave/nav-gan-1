/// Fog light: bulb, lens, bracket, switch
/// Phase 674

#[derive(Debug, Clone)]
pub struct FogLight {
    pub bulb_ok: bool,
    pub lens_ok: bool,
    pub bracket_ok: bool,
    pub switch_ok: bool,
    pub aligned: bool,
}

impl Default for FogLight {
    fn default() -> Self {
        Self::new()
    }
}

impl FogLight {
    pub fn new() -> Self {
        Self {
            bulb_ok: true,
            lens_ok: true,
            bracket_ok: true,
            switch_ok: true,
            aligned: true,
        }
    }

    pub fn lamp_ok(&self) -> bool {
        self.bulb_ok && self.lens_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.aligned
    }

    pub fn all_ok(&self) -> bool {
        self.lamp_ok() && self.mounting_ok() && self.switch_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.bulb_ok || !self.aligned
    }

    pub fn health_score(&self) -> f64 {
        if !self.bulb_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamp() {
        let c = FogLight::new();
        assert!(c.lamp_ok());
    }

    #[test]
    fn test_mounting() {
        let c = FogLight::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FogLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FogLight::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bulb() {
        let mut c = FogLight::new();
        c.bulb_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FogLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
