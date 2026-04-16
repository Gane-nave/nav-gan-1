/// Engine cover: acoustic insulation, thermal protection, aesthetics
/// Phase 361

#[derive(Debug, Clone)]
pub struct EngineCover {
    pub installed: bool,
    pub clips_ok: bool,
    pub insulation_ok: bool,
    pub heat_shield_ok: bool,
    pub cracked: bool,
}

impl Default for EngineCover {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineCover {
    pub fn new() -> Self {
        Self {
            installed: true,
            clips_ok: true,
            insulation_ok: true,
            heat_shield_ok: true,
            cracked: false,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.installed && self.clips_ok && !self.cracked
    }

    pub fn noise_reduction_ok(&self) -> bool {
        self.installed && self.insulation_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked
    }

    pub fn thermal_ok(&self) -> bool {
        self.heat_shield_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 20.0;
        }
        if !self.clips_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let e = EngineCover::new();
        assert!(e.all_ok());
    }

    #[test]
    fn test_noise() {
        let e = EngineCover::new();
        assert!(e.noise_reduction_ok());
    }

    #[test]
    fn test_no_replace() {
        let e = EngineCover::new();
        assert!(!e.needs_replacement());
    }

    #[test]
    fn test_thermal() {
        let e = EngineCover::new();
        assert!(e.thermal_ok());
    }

    #[test]
    fn test_cracked() {
        let mut e = EngineCover::new();
        e.cracked = true;
        assert!(e.needs_replacement());
    }

    #[test]
    fn test_health() {
        let e = EngineCover::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
