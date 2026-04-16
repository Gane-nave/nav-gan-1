/// Bolt torque monitoring: spec, actual, stretch, angle
/// Phase 798

#[derive(Debug, Clone)]
pub struct BoltTorque {
    pub spec_ok: bool,
    pub actual_ok: bool,
    pub stretch_ok: bool,
    pub angle_ok: bool,
    pub grade_ok: bool,
}

impl Default for BoltTorque {
    fn default() -> Self {
        Self::new()
    }
}

impl BoltTorque {
    pub fn new() -> Self {
        Self {
            spec_ok: true,
            actual_ok: true,
            stretch_ok: true,
            angle_ok: true,
            grade_ok: true,
        }
    }

    pub fn fastening_ok(&self) -> bool {
        self.spec_ok && self.actual_ok && self.angle_ok
    }

    pub fn material_ok(&self) -> bool {
        self.stretch_ok && self.grade_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fastening_ok() && self.material_ok()
    }

    pub fn needs_retorque(&self) -> bool {
        !self.actual_ok || !self.angle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actual_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastening() {
        let c = BoltTorque::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_material() {
        let c = BoltTorque::new();
        assert!(c.material_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BoltTorque::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retorque() {
        let c = BoltTorque::new();
        assert!(!c.needs_retorque());
    }

    #[test]
    fn test_actual() {
        let mut c = BoltTorque::new();
        c.actual_ok = false;
        assert!(c.needs_retorque());
    }

    #[test]
    fn test_health() {
        let c = BoltTorque::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
