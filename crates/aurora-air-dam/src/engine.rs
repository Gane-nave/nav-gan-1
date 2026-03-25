/// Air dam: front underbody airflow blockage, cooling duct integration
/// Phase 355

#[derive(Debug, Clone)]
pub struct AirDam {
    pub height_mm: f64,
    pub width_mm: f64,
    pub intact: bool,
    pub adjustable: bool,
    pub deployed: bool,
}

impl Default for AirDam {
    fn default() -> Self {
        Self::new()
    }
}

impl AirDam {
    pub fn new() -> Self {
        Self {
            height_mm: 100.0,
            width_mm: 1700.0,
            intact: true,
            adjustable: false,
            deployed: true,
        }
    }

    pub fn effective(&self) -> bool {
        self.intact && self.deployed
    }

    pub fn area_m2(&self) -> f64 {
        self.height_mm * self.width_mm / 1_000_000.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.intact
    }

    pub fn clearance_ok(&self) -> bool {
        self.height_mm > 50.0
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
    fn test_effective() {
        let a = AirDam::new();
        assert!(a.effective());
    }

    #[test]
    fn test_area() {
        let a = AirDam::new();
        assert!(a.area_m2() > 0.15);
    }

    #[test]
    fn test_no_replace() {
        let a = AirDam::new();
        assert!(!a.needs_replacement());
    }

    #[test]
    fn test_clearance() {
        let a = AirDam::new();
        assert!(a.clearance_ok());
    }

    #[test]
    fn test_broken() {
        let mut a = AirDam::new();
        a.intact = false;
        assert!(a.needs_replacement());
    }

    #[test]
    fn test_health() {
        let a = AirDam::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
