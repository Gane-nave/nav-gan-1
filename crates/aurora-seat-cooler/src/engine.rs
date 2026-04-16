/// Seat cooler: ventilated seat, thermoelectric, fan speed
/// Phase 453

#[derive(Debug, Clone)]
pub struct SeatCooler {
    pub level: u8,
    pub max_level: u8,
    pub fan_ok: bool,
    pub peltier_ok: bool,
    pub temp_c: f64,
}

impl Default for SeatCooler {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatCooler {
    pub fn new() -> Self {
        Self {
            level: 2,
            max_level: 3,
            fan_ok: true,
            peltier_ok: true,
            temp_c: 20.0,
        }
    }

    pub fn active(&self) -> bool {
        self.level > 0 && self.fan_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fan_ok && self.peltier_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.fan_ok || !self.peltier_ok
    }

    pub fn cooling_effective(&self) -> bool {
        self.temp_c < 25.0 && self.active()
    }

    pub fn health_score(&self) -> f64 {
        if !self.fan_ok {
            return 0.0;
        }
        if !self.peltier_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let s = SeatCooler::new();
        assert!(s.active());
    }

    #[test]
    fn test_all_ok() {
        let s = SeatCooler::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_service() {
        let s = SeatCooler::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_cooling() {
        let s = SeatCooler::new();
        assert!(s.cooling_effective());
    }

    #[test]
    fn test_bad_fan() {
        let mut s = SeatCooler::new();
        s.fan_ok = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SeatCooler::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
