/// Front splitter: airflow separation, pressure differential, ground effect
/// Phase 351

#[derive(Debug, Clone)]
pub struct Splitter {
    pub height_mm: f64,
    pub width_mm: f64,
    pub angle_deg: f64,
    pub intact: bool,
    pub mounted_ok: bool,
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Splitter {
    pub fn new() -> Self {
        Self {
            height_mm: 30.0,
            width_mm: 1800.0,
            angle_deg: 3.0,
            intact: true,
            mounted_ok: true,
        }
    }

    pub fn ground_clearance_ok(&self) -> bool {
        self.height_mm > 20.0
    }

    pub fn all_ok(&self) -> bool {
        self.intact && self.mounted_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.intact
    }

    pub fn area_m2(&self) -> f64 {
        self.height_mm * self.width_mm / 1_000_000.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if !self.mounted_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clearance() {
        let s = Splitter::new();
        assert!(s.ground_clearance_ok());
    }

    #[test]
    fn test_all_ok() {
        let s = Splitter::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let s = Splitter::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_area() {
        let s = Splitter::new();
        assert!(s.area_m2() > 0.04);
    }

    #[test]
    fn test_broken() {
        let mut s = Splitter::new();
        s.intact = false;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = Splitter::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
