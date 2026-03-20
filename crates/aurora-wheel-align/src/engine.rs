/// Wheel alignment: camber, caster, toe angles
/// Phase 484

#[derive(Debug, Clone)]
pub struct WheelAlignment {
    pub camber_deg: f64,
    pub caster_deg: f64,
    pub toe_deg: f64,
    pub within_spec: bool,
    pub adjusted: bool,
}

impl Default for WheelAlignment {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelAlignment {
    pub fn new() -> Self {
        Self {
            camber_deg: -0.5,
            caster_deg: 3.0,
            toe_deg: 0.1,
            within_spec: true,
            adjusted: true,
        }
    }

    pub fn camber_ok(&self) -> bool {
        (self.camber_deg).abs() < 2.0
    }

    pub fn caster_ok(&self) -> bool {
        self.caster_deg > 1.0 && self.caster_deg < 8.0
    }

    pub fn all_ok(&self) -> bool {
        self.camber_ok() && self.caster_ok() && self.within_spec
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.within_spec
    }

    pub fn health_score(&self) -> f64 {
        if !self.within_spec { return 40.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camber() {
        let c = WheelAlignment::new();
        assert!(c.camber_ok());
    }

    #[test]
    fn test_caster() {
        let c = WheelAlignment::new();
        assert!(c.caster_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelAlignment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = WheelAlignment::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_out_spec() {
        let mut c = WheelAlignment::new();
        c.within_spec = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = WheelAlignment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
