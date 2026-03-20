/// Air spring: air suspension, ride height, compressor
/// Phase 476

#[derive(Debug, Clone)]
pub struct AirSpring {
    pub pressure_bar: f64,
    pub ride_height_mm: f64,
    pub target_height_mm: f64,
    pub compressor_ok: bool,
    pub leak_detected: bool,
}

impl Default for AirSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl AirSpring {
    pub fn new() -> Self {
        Self {
            pressure_bar: 8.0,
            ride_height_mm: 150.0,
            target_height_mm: 150.0,
            compressor_ok: true,
            leak_detected: false,
        }
    }

    pub fn height_error_mm(&self) -> f64 {
        (self.ride_height_mm - self.target_height_mm).abs()
    }

    pub fn at_target(&self) -> bool {
        self.height_error_mm() < 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.compressor_ok && !self.leak_detected && self.at_target()
    }

    pub fn needs_service(&self) -> bool {
        self.leak_detected || !self.compressor_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_error() {
        let c = AirSpring::new();
        assert!(c.height_error_mm() < 1.0);
    }

    #[test]
    fn test_at_target() {
        let c = AirSpring::new();
        assert!(c.at_target());
    }

    #[test]
    fn test_all_ok() {
        let c = AirSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AirSpring::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = AirSpring::new();
        c.leak_detected = true;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AirSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
