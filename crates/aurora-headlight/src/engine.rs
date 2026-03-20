/// Headlight: beam pattern, leveling, adaptive lighting
/// Phase 531

#[derive(Debug, Clone)]
pub struct Headlight {
    pub brightness_lux: f64,
    pub min_brightness_lux: f64,
    pub leveling_ok: bool,
    pub lens_clear: bool,
    pub bulb_ok: bool,
}

impl Default for Headlight {
    fn default() -> Self {
        Self::new()
    }
}

impl Headlight {
    pub fn new() -> Self {
        Self {
            brightness_lux: 800.0,
            min_brightness_lux: 400.0,
            leveling_ok: true,
            lens_clear: true,
            bulb_ok: true,
        }
    }

    pub fn brightness_ok(&self) -> bool {
        self.brightness_lux > self.min_brightness_lux
    }

    pub fn optics_ok(&self) -> bool {
        self.lens_clear && self.leveling_ok
    }

    pub fn all_ok(&self) -> bool {
        self.brightness_ok() && self.optics_ok() && self.bulb_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.bulb_ok || !self.lens_clear
    }

    pub fn health_score(&self) -> f64 {
        if !self.bulb_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness() {
        let c = Headlight::new();
        assert!(c.brightness_ok());
    }

    #[test]
    fn test_optics() {
        let c = Headlight::new();
        assert!(c.optics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Headlight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Headlight::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bulb() {
        let mut c = Headlight::new();
        c.bulb_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Headlight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
