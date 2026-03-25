/// Washer load: load distribution, spring washer, Belleville washer
/// Phase 404

#[derive(Debug, Clone)]
pub struct WasherLoad {
    pub load_kn: f64,
    pub max_load_kn: f64,
    pub spring_type: bool,
    pub flat: bool,
    pub intact: bool,
}

impl Default for WasherLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl WasherLoad {
    pub fn new() -> Self {
        Self {
            load_kn: 5.0,
            max_load_kn: 15.0,
            spring_type: false,
            flat: true,
            intact: true,
        }
    }

    pub fn load_ok(&self) -> bool {
        self.load_kn <= self.max_load_kn
    }

    pub fn anti_vibration(&self) -> bool {
        self.spring_type
    }

    pub fn needs_replacement(&self) -> bool {
        !self.intact
    }

    pub fn load_pct(&self) -> f64 {
        if self.max_load_kn <= 0.0 {
            return 0.0;
        }
        (self.load_kn / self.max_load_kn * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let w = WasherLoad::new();
        assert!(w.load_ok());
    }

    #[test]
    fn test_no_anti_vib() {
        let w = WasherLoad::new();
        assert!(!w.anti_vibration());
    }

    #[test]
    fn test_no_replace() {
        let w = WasherLoad::new();
        assert!(!w.needs_replacement());
    }

    #[test]
    fn test_pct() {
        let w = WasherLoad::new();
        assert!(w.load_pct() < 40.0);
    }

    #[test]
    fn test_broken() {
        let mut w = WasherLoad::new();
        w.intact = false;
        assert!(w.needs_replacement());
    }

    #[test]
    fn test_health() {
        let w = WasherLoad::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
