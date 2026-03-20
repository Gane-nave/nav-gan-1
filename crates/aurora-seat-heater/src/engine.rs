/// Seat heater: heating element, temperature control, safety cutoff
/// Phase 452

#[derive(Debug, Clone)]
pub struct SeatHeater {
    pub level: u8,
    pub max_level: u8,
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub element_ok: bool,
}

impl Default for SeatHeater {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatHeater {
    pub fn new() -> Self {
        Self {
            level: 2,
            max_level: 3,
            temp_c: 35.0,
            max_temp_c: 45.0,
            element_ok: true,
        }
    }

    pub fn active(&self) -> bool {
        self.level > 0 && self.element_ok
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.element_ok && self.temp_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.element_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.element_ok {
            return 0.0;
        }
        if !self.temp_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let s = SeatHeater::new();
        assert!(s.active());
    }

    #[test]
    fn test_temp() {
        let s = SeatHeater::new();
        assert!(s.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let s = SeatHeater::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_service() {
        let s = SeatHeater::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_bad_element() {
        let mut s = SeatHeater::new();
        s.element_ok = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SeatHeater::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
