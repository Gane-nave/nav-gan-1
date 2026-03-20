/// Alternator: rotor, stator, regulator, diode
/// Phase 724

#[derive(Debug, Clone)]
pub struct Alternator {
    pub rotor_ok: bool,
    pub stator_ok: bool,
    pub regulator_ok: bool,
    pub diode_ok: bool,
    pub output_ok: bool,
}

impl Default for Alternator {
    fn default() -> Self {
        Self::new()
    }
}

impl Alternator {
    pub fn new() -> Self {
        Self {
            rotor_ok: true,
            stator_ok: true,
            regulator_ok: true,
            diode_ok: true,
            output_ok: true,
        }
    }

    pub fn generation_ok(&self) -> bool {
        self.rotor_ok && self.stator_ok && self.output_ok
    }

    pub fn regulation_ok(&self) -> bool {
        self.regulator_ok && self.diode_ok
    }

    pub fn all_ok(&self) -> bool {
        self.generation_ok() && self.regulation_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.rotor_ok || !self.diode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rotor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let c = Alternator::new();
        assert!(c.generation_ok());
    }

    #[test]
    fn test_regulation() {
        let c = Alternator::new();
        assert!(c.regulation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Alternator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Alternator::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_rotor() {
        let mut c = Alternator::new();
        c.rotor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Alternator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
