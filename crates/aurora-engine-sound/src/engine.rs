/// Engine sound: order analysis, harmonic content, sound quality index
/// Phase 370

#[derive(Debug, Clone)]
pub struct EngineSound {
    pub rpm: f64,
    pub dominant_order: f64,
    pub sqi: f64,
    pub booming: bool,
    pub roughness: f64,
}

impl Default for EngineSound {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineSound {
    pub fn new() -> Self {
        Self {
            rpm: 2000.0,
            dominant_order: 2.0,
            sqi: 85.0,
            booming: false,
            roughness: 0.3,
        }
    }

    pub fn quality_ok(&self) -> bool {
        self.sqi > 70.0
    }

    pub fn premium_quality(&self) -> bool {
        self.sqi > 90.0
    }

    pub fn smooth(&self) -> bool {
        self.roughness < 0.5 && !self.booming
    }

    pub fn needs_tuning(&self) -> bool {
        self.booming || self.sqi < 60.0
    }

    pub fn health_score(&self) -> f64 {
        if self.booming {
            return 30.0;
        }
        if self.sqi < 50.0 {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality() {
        let e = EngineSound::new();
        assert!(e.quality_ok());
    }

    #[test]
    fn test_not_premium() {
        let e = EngineSound::new();
        assert!(!e.premium_quality());
    }

    #[test]
    fn test_smooth() {
        let e = EngineSound::new();
        assert!(e.smooth());
    }

    #[test]
    fn test_no_tuning() {
        let e = EngineSound::new();
        assert!(!e.needs_tuning());
    }

    #[test]
    fn test_boom() {
        let mut e = EngineSound::new();
        e.booming = true;
        assert!(e.needs_tuning());
    }

    #[test]
    fn test_health() {
        let e = EngineSound::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
