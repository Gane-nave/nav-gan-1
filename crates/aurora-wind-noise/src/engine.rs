/// Wind noise: aeroacoustic analysis, mirror/pillar turbulence, sealing
/// Phase 366

#[derive(Debug, Clone)]
pub struct WindNoise {
    pub noise_db: f64,
    pub speed_kmh: f64,
    pub mirror_contrib_db: f64,
    pub pillar_contrib_db: f64,
    pub sealing_ok: bool,
}

impl Default for WindNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl WindNoise {
    pub fn new() -> Self {
        Self {
            noise_db: 35.0,
            speed_kmh: 100.0,
            mirror_contrib_db: 5.0,
            pillar_contrib_db: 3.0,
            sealing_ok: true,
        }
    }

    pub fn acceptable(&self) -> bool {
        self.noise_db < 45.0
    }

    pub fn premium(&self) -> bool {
        self.noise_db < 32.0
    }

    pub fn dominant_source(&self) -> &str {
        if self.mirror_contrib_db > self.pillar_contrib_db {
            "mirror"
        } else {
            "pillar"
        }
    }

    pub fn seal_effective(&self) -> bool {
        self.sealing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sealing_ok {
            return 30.0;
        }
        if self.noise_db > 50.0 {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable() {
        let w = WindNoise::new();
        assert!(w.acceptable());
    }

    #[test]
    fn test_not_premium() {
        let w = WindNoise::new();
        assert!(!w.premium());
    }

    #[test]
    fn test_source() {
        let w = WindNoise::new();
        assert_eq!(w.dominant_source(), "mirror");
    }

    #[test]
    fn test_seal() {
        let w = WindNoise::new();
        assert!(w.seal_effective());
    }

    #[test]
    fn test_loud() {
        let mut w = WindNoise::new();
        w.noise_db = 50.0;
        assert!(!w.acceptable());
    }

    #[test]
    fn test_health() {
        let w = WindNoise::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
