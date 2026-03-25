/// Winch: motor, cable, drum, controller, brake
/// Phase 847

#[derive(Debug, Clone)]
pub struct Winch {
    pub motor_ok: bool,
    pub cable_ok: bool,
    pub drum_ok: bool,
    pub controller_ok: bool,
    pub brake_ok: bool,
}

impl Default for Winch {
    fn default() -> Self {
        Self::new()
    }
}

impl Winch {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            cable_ok: true,
            drum_ok: true,
            controller_ok: true,
            brake_ok: true,
        }
    }

    pub fn power_ok(&self) -> bool {
        self.motor_ok && self.controller_ok
    }

    pub fn rigging_ok(&self) -> bool {
        self.cable_ok && self.drum_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.power_ok() && self.rigging_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.cable_ok || !self.motor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cable_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let c = Winch::new();
        assert!(c.power_ok());
    }

    #[test]
    fn test_rigging() {
        let c = Winch::new();
        assert!(c.rigging_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Winch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Winch::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_cable() {
        let mut c = Winch::new();
        c.cable_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Winch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
