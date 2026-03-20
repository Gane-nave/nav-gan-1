/// Reducer: gear ratio, bearing, oil, housing
/// Phase 863

#[derive(Debug, Clone)]
pub struct Reducer {
    pub ratio_ok: bool,
    pub bearing_ok: bool,
    pub oil_ok: bool,
    pub housing_ok: bool,
    pub noise_free: bool,
}

impl Default for Reducer {
    fn default() -> Self {
        Self::new()
    }
}

impl Reducer {
    pub fn new() -> Self {
        Self {
            ratio_ok: true,
            bearing_ok: true,
            oil_ok: true,
            housing_ok: true,
            noise_free: true,
        }
    }

    pub fn drive_ok(&self) -> bool {
        self.ratio_ok && self.bearing_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.oil_ok && self.housing_ok && self.noise_free
    }

    pub fn all_ok(&self) -> bool {
        self.drive_ok() && self.condition_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bearing_ok || !self.oil_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive() {
        let c = Reducer::new();
        assert!(c.drive_ok());
    }

    #[test]
    fn test_condition() {
        let c = Reducer::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Reducer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Reducer::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bearing() {
        let mut c = Reducer::new();
        c.bearing_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Reducer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
