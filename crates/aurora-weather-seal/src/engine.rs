/// Weather seal: door, window, trunk, compression
/// Phase 768

#[derive(Debug, Clone)]
pub struct WeatherSeal {
    pub door_ok: bool,
    pub window_ok: bool,
    pub trunk_ok: bool,
    pub compression_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for WeatherSeal {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherSeal {
    pub fn new() -> Self {
        Self {
            door_ok: true,
            window_ok: true,
            trunk_ok: true,
            compression_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.door_ok && self.window_ok && self.trunk_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.compression_ok && self.adhesion_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.compression_ok || !self.door_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compression_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = WeatherSeal::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_condition() {
        let c = WeatherSeal::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WeatherSeal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WeatherSeal::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_compression() {
        let mut c = WeatherSeal::new();
        c.compression_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WeatherSeal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
