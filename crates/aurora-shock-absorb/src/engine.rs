/// Shock absorber: monotube/twin-tube, rebound/compression damping
/// Phase 331

#[derive(Debug, Clone)]
pub struct ShockAbsorber {
    pub rebound_force_n: f64,
    pub compression_force_n: f64,
    pub leaking: bool,
    pub worn: bool,
    pub adaptive: bool,
}

impl Default for ShockAbsorber {
    fn default() -> Self {
        Self::new()
    }
}

impl ShockAbsorber {
    pub fn new() -> Self {
        Self {
            rebound_force_n: 800.0,
            compression_force_n: 400.0,
            leaking: false,
            worn: false,
            adaptive: true,
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.compression_force_n <= 0.0 {
            return 0.0;
        }
        self.rebound_force_n / self.compression_force_n
    }

    pub fn functioning(&self) -> bool {
        !self.leaking && !self.worn
    }

    pub fn needs_replacement(&self) -> bool {
        self.leaking || self.worn
    }

    pub fn ratio_ok(&self) -> bool {
        let r = self.ratio();
        r > 1.5 && r < 3.0
    }

    pub fn health_score(&self) -> f64 {
        if self.leaking {
            return 0.0;
        }
        if self.worn {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio() {
        let s = ShockAbsorber::new();
        assert!((s.ratio() - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_functioning() {
        let s = ShockAbsorber::new();
        assert!(s.functioning());
    }

    #[test]
    fn test_no_replace() {
        let s = ShockAbsorber::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_ratio_ok() {
        let s = ShockAbsorber::new();
        assert!(s.ratio_ok());
    }

    #[test]
    fn test_leak() {
        let mut s = ShockAbsorber::new();
        s.leaking = true;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = ShockAbsorber::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
