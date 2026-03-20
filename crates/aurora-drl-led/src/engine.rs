/// DRL LED: daytime running light, LED strip, light guide, intensity
/// Phase 424

#[derive(Debug, Clone)]
pub struct DrlLed {
    pub intensity_cd: f64,
    pub min_cd: f64,
    pub uniform: bool,
    pub auto_dim: bool,
    pub led_failures: u32,
}

impl Default for DrlLed {
    fn default() -> Self {
        Self::new()
    }
}

impl DrlLed {
    pub fn new() -> Self {
        Self {
            intensity_cd: 800.0,
            min_cd: 400.0,
            uniform: true,
            auto_dim: true,
            led_failures: 0,
        }
    }

    pub fn intensity_ok(&self) -> bool {
        self.intensity_cd >= self.min_cd
    }

    pub fn all_ok(&self) -> bool {
        self.intensity_ok() && self.uniform && self.led_failures == 0
    }

    pub fn compliant(&self) -> bool {
        self.intensity_ok() && self.auto_dim
    }

    pub fn needs_service(&self) -> bool {
        self.led_failures > 2 || !self.intensity_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.intensity_ok() {
            return 20.0;
        }
        if self.led_failures > 0 {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intensity() {
        let d = DrlLed::new();
        assert!(d.intensity_ok());
    }

    #[test]
    fn test_all_ok() {
        let d = DrlLed::new();
        assert!(d.all_ok());
    }

    #[test]
    fn test_compliant() {
        let d = DrlLed::new();
        assert!(d.compliant());
    }

    #[test]
    fn test_no_service() {
        let d = DrlLed::new();
        assert!(!d.needs_service());
    }

    #[test]
    fn test_dim() {
        let mut d = DrlLed::new();
        d.intensity_cd = 200.0;
        assert!(d.needs_service());
    }

    #[test]
    fn test_health() {
        let d = DrlLed::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
