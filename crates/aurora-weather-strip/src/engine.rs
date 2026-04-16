/// Weather strip: EPDM rubber, compression set resistance, UV aging
/// Phase 373

#[derive(Debug, Clone)]
pub struct WeatherStrip {
    pub compression_set_pct: f64,
    pub uv_degraded: bool,
    pub hardness_shore: f64,
    pub intact: bool,
    pub seal_force_n: f64,
}

impl Default for WeatherStrip {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherStrip {
    pub fn new() -> Self {
        Self {
            compression_set_pct: 15.0,
            uv_degraded: false,
            hardness_shore: 55.0,
            intact: true,
            seal_force_n: 8.0,
        }
    }

    pub fn effective(&self) -> bool {
        self.intact && self.compression_set_pct < 50.0
    }

    pub fn hardness_ok(&self) -> bool {
        self.hardness_shore > 40.0 && self.hardness_shore < 75.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.intact || self.uv_degraded || self.compression_set_pct > 60.0
    }

    pub fn seal_ok(&self) -> bool {
        self.seal_force_n > 3.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if self.uv_degraded {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let w = WeatherStrip::new();
        assert!(w.effective());
    }

    #[test]
    fn test_hardness() {
        let w = WeatherStrip::new();
        assert!(w.hardness_ok());
    }

    #[test]
    fn test_no_replace() {
        let w = WeatherStrip::new();
        assert!(!w.needs_replacement());
    }

    #[test]
    fn test_seal() {
        let w = WeatherStrip::new();
        assert!(w.seal_ok());
    }

    #[test]
    fn test_degraded() {
        let mut w = WeatherStrip::new();
        w.uv_degraded = true;
        assert!(w.needs_replacement());
    }

    #[test]
    fn test_health() {
        let w = WeatherStrip::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
