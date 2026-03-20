/// Transmission controller: gear selection, shift quality, adaptation
/// Phase 327

#[derive(Debug, Clone)]
pub struct TransmissionController {
    pub current_gear: u8,
    pub gear_count: u8,
    pub shift_quality_pct: f64,
    pub fluid_temp_c: f64,
    pub fluid_level_ok: bool,
    pub solenoids_ok: bool,
}

impl Default for TransmissionController {
    fn default() -> Self {
        Self::new()
    }
}

impl TransmissionController {
    pub fn new() -> Self {
        Self {
            current_gear: 3,
            gear_count: 8,
            shift_quality_pct: 95.0,
            fluid_temp_c: 80.0,
            fluid_level_ok: true,
            solenoids_ok: true,
        }
    }

    pub fn in_gear(&self) -> bool {
        (1..=self.gear_count).contains(&self.current_gear)
    }

    pub fn shift_quality_ok(&self) -> bool {
        self.shift_quality_pct > 80.0
    }

    pub fn temp_ok(&self) -> bool {
        self.fluid_temp_c < 120.0
    }

    pub fn needs_service(&self) -> bool {
        !self.fluid_level_ok || !self.solenoids_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.solenoids_ok {
            return 10.0;
        }
        if !self.fluid_level_ok {
            return 30.0;
        }
        if !self.temp_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_gear() {
        let t = TransmissionController::new();
        assert!(t.in_gear());
    }

    #[test]
    fn test_shift_ok() {
        let t = TransmissionController::new();
        assert!(t.shift_quality_ok());
    }

    #[test]
    fn test_temp() {
        let t = TransmissionController::new();
        assert!(t.temp_ok());
    }

    #[test]
    fn test_no_service() {
        let t = TransmissionController::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_bad_solenoid() {
        let mut t = TransmissionController::new();
        t.solenoids_ok = false;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TransmissionController::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
