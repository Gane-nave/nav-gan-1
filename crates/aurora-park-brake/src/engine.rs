/// Parking brake: cable, actuator, adjustment, indicator
/// Phase 661

#[derive(Debug, Clone)]
pub struct ParkBrake {
    pub cable_ok: bool,
    pub actuator_ok: bool,
    pub adjusted: bool,
    pub indicator_ok: bool,
    pub holding: bool,
}

impl Default for ParkBrake {
    fn default() -> Self {
        Self::new()
    }
}

impl ParkBrake {
    pub fn new() -> Self {
        Self {
            cable_ok: true,
            actuator_ok: true,
            adjusted: true,
            indicator_ok: true,
            holding: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.cable_ok && self.actuator_ok
    }

    pub fn function_ok(&self) -> bool {
        self.adjusted && self.holding
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.function_ok() && self.indicator_ok
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.adjusted || !self.cable_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.holding {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = ParkBrake::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_function() {
        let c = ParkBrake::new();
        assert!(c.function_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ParkBrake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = ParkBrake::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_cable() {
        let mut c = ParkBrake::new();
        c.cable_ok = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = ParkBrake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
