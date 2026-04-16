/// Tail light: LED array, lens, seal, harness
/// Phase 673

#[derive(Debug, Clone)]
pub struct TailLight {
    pub led_ok: bool,
    pub lens_ok: bool,
    pub seal_ok: bool,
    pub harness_ok: bool,
    pub brightness_ok: bool,
}

impl Default for TailLight {
    fn default() -> Self {
        Self::new()
    }
}

impl TailLight {
    pub fn new() -> Self {
        Self {
            led_ok: true,
            lens_ok: true,
            seal_ok: true,
            harness_ok: true,
            brightness_ok: true,
        }
    }

    pub fn lamp_ok(&self) -> bool {
        self.led_ok && self.brightness_ok
    }

    pub fn housing_ok(&self) -> bool {
        self.lens_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.lamp_ok() && self.housing_ok() && self.harness_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.led_ok || !self.lens_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.led_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamp() {
        let c = TailLight::new();
        assert!(c.lamp_ok());
    }

    #[test]
    fn test_housing() {
        let c = TailLight::new();
        assert!(c.housing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TailLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TailLight::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_led() {
        let mut c = TailLight::new();
        c.led_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TailLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
