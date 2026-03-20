/// Gear whine: mesh frequency, tooth profile, transmission noise
/// Phase 379

#[derive(Debug, Clone)]
pub struct GearWhine {
    pub frequency_hz: f64,
    pub amplitude_db: f64,
    pub gear_pair: u8,
    pub lubrication_ok: bool,
    pub profile_ok: bool,
}

impl Default for GearWhine {
    fn default() -> Self {
        Self::new()
    }
}

impl GearWhine {
    pub fn new() -> Self {
        Self {
            frequency_hz: 800.0,
            amplitude_db: 20.0,
            gear_pair: 3,
            lubrication_ok: true,
            profile_ok: true,
        }
    }

    pub fn audible(&self) -> bool {
        self.amplitude_db > 25.0
    }

    pub fn acceptable(&self) -> bool {
        self.amplitude_db < 35.0
    }

    pub fn needs_attention(&self) -> bool {
        !self.lubrication_ok || !self.profile_ok
    }

    pub fn high_frequency(&self) -> bool {
        self.frequency_hz > 2000.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.profile_ok {
            return 20.0;
        }
        if !self.lubrication_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_audible() {
        let g = GearWhine::new();
        assert!(!g.audible());
    }

    #[test]
    fn test_acceptable() {
        let g = GearWhine::new();
        assert!(g.acceptable());
    }

    #[test]
    fn test_no_attention() {
        let g = GearWhine::new();
        assert!(!g.needs_attention());
    }

    #[test]
    fn test_not_high_freq() {
        let g = GearWhine::new();
        assert!(!g.high_frequency());
    }

    #[test]
    fn test_bad_profile() {
        let mut g = GearWhine::new();
        g.profile_ok = false;
        assert!(g.needs_attention());
    }

    #[test]
    fn test_health() {
        let g = GearWhine::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
