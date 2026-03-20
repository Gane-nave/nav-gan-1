/// Sound generator: AVAS, speed-dependent, pedestrian, sport
/// Phase 882

#[derive(Debug, Clone)]
pub struct SoundGen {
    pub avas_ok: bool,
    pub speed_ok: bool,
    pub pedestrian_ok: bool,
    pub sport_ok: bool,
    pub volume_ok: bool,
}

impl Default for SoundGen {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundGen {
    pub fn new() -> Self {
        Self {
            avas_ok: true,
            speed_ok: true,
            pedestrian_ok: true,
            sport_ok: true,
            volume_ok: true,
        }
    }

    pub fn compliance_ok(&self) -> bool {
        self.avas_ok && self.pedestrian_ok
    }

    pub fn features_ok(&self) -> bool {
        self.speed_ok && self.sport_ok && self.volume_ok
    }

    pub fn all_ok(&self) -> bool {
        self.compliance_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.avas_ok || !self.pedestrian_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.avas_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance() {
        let c = SoundGen::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_features() {
        let c = SoundGen::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SoundGen::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SoundGen::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_avas() {
        let mut c = SoundGen::new();
        c.avas_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SoundGen::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
