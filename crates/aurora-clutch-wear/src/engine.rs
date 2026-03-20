/// Clutch wear: friction disc, pressure plate, throw-out bearing
/// Phase 456

#[derive(Debug, Clone)]
pub struct ClutchWear {
    pub thickness_mm: f64,
    pub min_thickness_mm: f64,
    pub slip_pct: f64,
    pub spring_ok: bool,
    pub bearing_ok: bool,
}

impl Default for ClutchWear {
    fn default() -> Self {
        Self::new()
    }
}

impl ClutchWear {
    pub fn new() -> Self {
        Self {
            thickness_mm: 8.0,
            min_thickness_mm: 3.0,
            slip_pct: 0.5,
            spring_ok: true,
            bearing_ok: true,
        }
    }

    pub fn thickness_ok(&self) -> bool {
        self.thickness_mm > self.min_thickness_mm
    }

    pub fn no_slip(&self) -> bool {
        self.slip_pct < 3.0
    }

    pub fn all_ok(&self) -> bool {
        self.thickness_ok() && self.no_slip() && self.spring_ok && self.bearing_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.thickness_ok() || self.slip_pct > 10.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.thickness_ok() {
            return 10.0;
        }
        if !self.bearing_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thickness() {
        let c = ClutchWear::new();
        assert!(c.thickness_ok());
    }

    #[test]
    fn test_no_slip() {
        let c = ClutchWear::new();
        assert!(c.no_slip());
    }

    #[test]
    fn test_all_ok() {
        let c = ClutchWear::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ClutchWear::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_worn() {
        let mut c = ClutchWear::new();
        c.thickness_mm = 2.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ClutchWear::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
