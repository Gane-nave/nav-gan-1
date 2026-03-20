/// Final drive: ring & pinion, gear ratio, backlash, bearing preload
/// Phase 470

#[derive(Debug, Clone)]
pub struct FinalDrive {
    pub ratio: f64,
    pub backlash_mm: f64,
    pub max_backlash_mm: f64,
    pub bearing_ok: bool,
    pub oil_ok: bool,
}

impl Default for FinalDrive {
    fn default() -> Self {
        Self::new()
    }
}

impl FinalDrive {
    pub fn new() -> Self {
        Self {
            ratio: 3.73,
            backlash_mm: 0.15,
            max_backlash_mm: 0.30,
            bearing_ok: true,
            oil_ok: true,
        }
    }

    pub fn backlash_ok(&self) -> bool {
        self.backlash_mm < self.max_backlash_mm
    }

    pub fn all_ok(&self) -> bool {
        self.backlash_ok() && self.bearing_ok && self.oil_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_ok || !self.backlash_ok()
    }

    pub fn quiet(&self) -> bool {
        self.backlash_ok() && self.bearing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.backlash_ok() {
            return 20.0;
        }
        if !self.bearing_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backlash() {
        let f = FinalDrive::new();
        assert!(f.backlash_ok());
    }

    #[test]
    fn test_all_ok() {
        let f = FinalDrive::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_no_service() {
        let f = FinalDrive::new();
        assert!(!f.needs_service());
    }

    #[test]
    fn test_quiet() {
        let f = FinalDrive::new();
        assert!(f.quiet());
    }

    #[test]
    fn test_excess_backlash() {
        let mut f = FinalDrive::new();
        f.backlash_mm = 0.5;
        assert!(f.needs_service());
    }

    #[test]
    fn test_health() {
        let f = FinalDrive::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
