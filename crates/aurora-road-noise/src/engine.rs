/// Road noise: tire-road interaction, structure-borne, airborne
/// Phase 367

#[derive(Debug, Clone)]
pub struct RoadNoise {
    pub structure_db: f64,
    pub airborne_db: f64,
    pub surface_type: u8,
    pub insulation_ok: bool,
}

impl Default for RoadNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadNoise {
    pub fn new() -> Self {
        Self {
            structure_db: 30.0,
            airborne_db: 25.0,
            surface_type: 1,
            insulation_ok: true,
        }
    }

    pub fn total_db(&self) -> f64 {
        (10.0_f64.powf(self.structure_db / 10.0) + 10.0_f64.powf(self.airborne_db / 10.0)).log10()
            * 10.0
    }

    pub fn acceptable(&self) -> bool {
        self.total_db() < 40.0
    }

    pub fn dominant_path(&self) -> &str {
        if self.structure_db > self.airborne_db {
            "structure"
        } else {
            "airborne"
        }
    }

    pub fn smooth_surface(&self) -> bool {
        self.surface_type <= 1
    }

    pub fn health_score(&self) -> f64 {
        if !self.insulation_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        let r = RoadNoise::new();
        assert!(r.total_db() > 30.0);
    }

    #[test]
    fn test_acceptable() {
        let r = RoadNoise::new();
        assert!(r.acceptable());
    }

    #[test]
    fn test_dominant() {
        let r = RoadNoise::new();
        assert_eq!(r.dominant_path(), "structure");
    }

    #[test]
    fn test_smooth() {
        let r = RoadNoise::new();
        assert!(r.smooth_surface());
    }

    #[test]
    fn test_bad_insulation() {
        let mut r = RoadNoise::new();
        r.insulation_ok = false;
        assert!((r.health_score() - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_health() {
        let r = RoadNoise::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
