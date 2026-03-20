/// Seat heater: element, thermostat, zone, timer
/// Phase 748

#[derive(Debug, Clone)]
pub struct SeatHeat {
    pub element_ok: bool,
    pub thermostat_ok: bool,
    pub zone_ok: bool,
    pub timer_ok: bool,
    pub safety_ok: bool,
}

impl Default for SeatHeat {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatHeat {
    pub fn new() -> Self {
        Self {
            element_ok: true,
            thermostat_ok: true,
            zone_ok: true,
            timer_ok: true,
            safety_ok: true,
        }
    }

    pub fn heating_ok(&self) -> bool {
        self.element_ok && self.thermostat_ok
    }

    pub fn control_ok(&self) -> bool {
        self.zone_ok && self.timer_ok && self.safety_ok
    }

    pub fn all_ok(&self) -> bool {
        self.heating_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.element_ok || !self.thermostat_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.element_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heating() {
        let c = SeatHeat::new();
        assert!(c.heating_ok());
    }

    #[test]
    fn test_control() {
        let c = SeatHeat::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatHeat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SeatHeat::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_element() {
        let mut c = SeatHeat::new();
        c.element_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SeatHeat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
