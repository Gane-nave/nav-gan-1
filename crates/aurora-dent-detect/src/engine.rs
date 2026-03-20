/// Dent detection: PDR eligibility, depth/area measurement, panel stress
/// Phase 392

#[derive(Debug, Clone)]
pub struct DentDetect {
    pub depth_mm: f64,
    pub diameter_mm: f64,
    pub paint_intact: bool,
    pub crease: bool,
    pub count: u32,
}

impl Default for DentDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl DentDetect {
    pub fn new() -> Self {
        Self {
            depth_mm: 1.0,
            diameter_mm: 15.0,
            paint_intact: true,
            crease: false,
            count: 1,
        }
    }

    pub fn pdr_eligible(&self) -> bool {
        self.paint_intact && !self.crease && self.depth_mm < 10.0
    }

    pub fn minor(&self) -> bool {
        self.depth_mm < 3.0 && self.diameter_mm < 30.0
    }

    pub fn needs_body_work(&self) -> bool {
        self.crease || self.depth_mm > 15.0 || !self.paint_intact
    }

    pub fn area_mm2(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_mm / 2.0).powi(2)
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_body_work() {
            return 20.0;
        }
        if !self.minor() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdr() {
        let d = DentDetect::new();
        assert!(d.pdr_eligible());
    }

    #[test]
    fn test_minor() {
        let d = DentDetect::new();
        assert!(d.minor());
    }

    #[test]
    fn test_no_body() {
        let d = DentDetect::new();
        assert!(!d.needs_body_work());
    }

    #[test]
    fn test_area() {
        let d = DentDetect::new();
        assert!(d.area_mm2() > 150.0);
    }

    #[test]
    fn test_crease() {
        let mut d = DentDetect::new();
        d.crease = true;
        assert!(d.needs_body_work());
    }

    #[test]
    fn test_health() {
        let d = DentDetect::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
