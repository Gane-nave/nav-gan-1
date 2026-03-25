/// Gear selection: shift-by-wire, gear position sensor, interlock
/// Phase 465

#[derive(Debug, Clone)]
pub struct GearSelect {
    pub position: u8,
    pub sensor_ok: bool,
    pub interlock_ok: bool,
    pub actuator_ok: bool,
    pub display_ok: bool,
}

impl Default for GearSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl GearSelect {
    pub fn new() -> Self {
        Self {
            position: 4,
            sensor_ok: true,
            interlock_ok: true,
            actuator_ok: true,
            display_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok && self.interlock_ok && self.actuator_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.interlock_ok && self.sensor_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.actuator_ok
    }

    pub fn in_park(&self) -> bool {
        self.position == 0
    }

    pub fn health_score(&self) -> f64 {
        if !self.interlock_ok {
            return 0.0;
        }
        if !self.sensor_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let g = GearSelect::new();
        assert!(g.all_ok());
    }

    #[test]
    fn test_safety() {
        let g = GearSelect::new();
        assert!(g.safety_ok());
    }

    #[test]
    fn test_no_service() {
        let g = GearSelect::new();
        assert!(!g.needs_service());
    }

    #[test]
    fn test_not_park() {
        let g = GearSelect::new();
        assert!(!g.in_park());
    }

    #[test]
    fn test_bad_sensor() {
        let mut g = GearSelect::new();
        g.sensor_ok = false;
        assert!(g.needs_service());
    }

    #[test]
    fn test_health() {
        let g = GearSelect::new();
        assert!((g.health_score() - 100.0).abs() < 0.1);
    }
}
