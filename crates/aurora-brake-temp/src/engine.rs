/// Brake temperature: rotor, pad, fluid, cooling, fade
/// Phase 952

#[derive(Debug, Clone)]
pub struct BrakeTemp {
    pub rotor_ok: bool,
    pub pad_ok: bool,
    pub fluid_ok: bool,
    pub cooling_ok: bool,
    pub fade_ok: bool,
}

impl Default for BrakeTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeTemp {
    pub fn new() -> Self {
        Self {
            rotor_ok: true,
            pad_ok: true,
            fluid_ok: true,
            cooling_ok: true,
            fade_ok: true,
        }
    }

    pub fn monitoring_ok(&self) -> bool {
        self.rotor_ok && self.pad_ok && self.fluid_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.cooling_ok && self.fade_ok
    }

    pub fn all_ok(&self) -> bool {
        self.monitoring_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.rotor_ok || !self.pad_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rotor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring() {
        let c = BrakeTemp::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_safety() {
        let c = BrakeTemp::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BrakeTemp::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_rotor() {
        let mut c = BrakeTemp::new();
        c.rotor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BrakeTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
