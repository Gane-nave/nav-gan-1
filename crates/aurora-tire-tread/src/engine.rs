/// Tire tread: depth, wear pattern, age, pressure
/// Phase 546

#[derive(Debug, Clone)]
pub struct TireTread {
    pub depth_mm: f64,
    pub min_depth_mm: f64,
    pub wear_even: bool,
    pub age_years: f64,
    pub pressure_ok: bool,
}

impl Default for TireTread {
    fn default() -> Self {
        Self::new()
    }
}

impl TireTread {
    pub fn new() -> Self {
        Self {
            depth_mm: 6.0,
            min_depth_mm: 1.6,
            wear_even: true,
            age_years: 2.0,
            pressure_ok: true,
        }
    }

    pub fn depth_ok(&self) -> bool {
        self.depth_mm > self.min_depth_mm
    }

    pub fn wear_ok(&self) -> bool {
        self.wear_even && self.age_years < 6.0
    }

    pub fn all_ok(&self) -> bool {
        self.depth_ok() && self.wear_ok() && self.pressure_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.depth_ok() || self.age_years > 5.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.depth_ok() { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth() {
        let c = TireTread::new();
        assert!(c.depth_ok());
    }

    #[test]
    fn test_wear() {
        let c = TireTread::new();
        assert!(c.wear_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TireTread::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TireTread::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_worn() {
        let mut c = TireTread::new();
        c.depth_mm = 1.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TireTread::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
