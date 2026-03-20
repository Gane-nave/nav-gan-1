/// NVH simulation: vibration, acoustic, isolation, damping
/// Phase 963

#[derive(Debug, Clone)]
pub struct NoiseSim {
    pub vibration_ok: bool,
    pub acoustic_ok: bool,
    pub isolation_ok: bool,
    pub damping_ok: bool,
    pub validate_ok: bool,
}

impl Default for NoiseSim {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseSim {
    pub fn new() -> Self {
        Self {
            vibration_ok: true,
            acoustic_ok: true,
            isolation_ok: true,
            damping_ok: true,
            validate_ok: true,
        }
    }

    pub fn analysis_ok(&self) -> bool {
        self.vibration_ok && self.acoustic_ok
    }

    pub fn mitigation_ok(&self) -> bool {
        self.isolation_ok && self.damping_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.analysis_ok() && self.mitigation_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.validate_ok || !self.vibration_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vibration_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis() {
        let c = NoiseSim::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_mitigation() {
        let c = NoiseSim::new();
        assert!(c.mitigation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NoiseSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = NoiseSim::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_validate() {
        let mut c = NoiseSim::new();
        c.validate_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = NoiseSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
