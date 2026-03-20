/// Weld integrity: spot weld, seam weld, nugget diameter, shear strength
/// Phase 396

#[derive(Debug, Clone)]
pub struct WeldInteg {
    pub nugget_mm: f64,
    pub min_nugget_mm: f64,
    pub shear_kn: f64,
    pub min_shear_kn: f64,
    pub defect_detected: bool,
}

impl Default for WeldInteg {
    fn default() -> Self {
        Self::new()
    }
}

impl WeldInteg {
    pub fn new() -> Self {
        Self {
            nugget_mm: 6.0,
            min_nugget_mm: 4.0,
            shear_kn: 8.0,
            min_shear_kn: 5.0,
            defect_detected: false,
        }
    }

    pub fn nugget_ok(&self) -> bool {
        self.nugget_mm >= self.min_nugget_mm
    }

    pub fn strength_ok(&self) -> bool {
        self.shear_kn >= self.min_shear_kn
    }

    pub fn all_ok(&self) -> bool {
        self.nugget_ok() && self.strength_ok() && !self.defect_detected
    }

    pub fn needs_reweld(&self) -> bool {
        self.defect_detected || !self.nugget_ok()
    }

    pub fn health_score(&self) -> f64 {
        if self.defect_detected {
            return 0.0;
        }
        if !self.strength_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nugget() {
        let w = WeldInteg::new();
        assert!(w.nugget_ok());
    }

    #[test]
    fn test_strength() {
        let w = WeldInteg::new();
        assert!(w.strength_ok());
    }

    #[test]
    fn test_all_ok() {
        let w = WeldInteg::new();
        assert!(w.all_ok());
    }

    #[test]
    fn test_no_reweld() {
        let w = WeldInteg::new();
        assert!(!w.needs_reweld());
    }

    #[test]
    fn test_defect() {
        let mut w = WeldInteg::new();
        w.defect_detected = true;
        assert!(w.needs_reweld());
    }

    #[test]
    fn test_health() {
        let w = WeldInteg::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
