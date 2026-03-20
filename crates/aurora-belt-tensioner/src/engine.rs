/// Belt tensioner: spring, pulley, bearing, deflection
/// Phase 575

#[derive(Debug, Clone)]
pub struct BeltTensioner {
    pub spring_ok: bool,
    pub pulley_ok: bool,
    pub bearing_ok: bool,
    pub deflection_mm: f64,
    pub max_deflection_mm: f64,
}

impl Default for BeltTensioner {
    fn default() -> Self {
        Self::new()
    }
}

impl BeltTensioner {
    pub fn new() -> Self {
        Self {
            spring_ok: true,
            pulley_ok: true,
            bearing_ok: true,
            deflection_mm: 8.0,
            max_deflection_mm: 15.0,
        }
    }

    pub fn deflection_ok(&self) -> bool {
        self.deflection_mm < self.max_deflection_mm
    }

    pub fn mechanical_ok(&self) -> bool {
        self.spring_ok && self.pulley_ok && self.bearing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.deflection_ok() && self.mechanical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.spring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deflection() {
        let c = BeltTensioner::new();
        assert!(c.deflection_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = BeltTensioner::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BeltTensioner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BeltTensioner::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing() {
        let mut c = BeltTensioner::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BeltTensioner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
