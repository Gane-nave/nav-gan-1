/// Ignition control: spark timing, coil-on-plug, misfire detection
/// Phase 305

#[derive(Debug, Clone)]
pub struct IgnitionController {
    pub timing_deg: f64,
    pub dwell_ms: f64,
    pub coils_ok: u8,
    pub total_coils: u8,
    pub misfire_count: u32,
    pub system_ok: bool,
}

impl Default for IgnitionController {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnitionController {
    pub fn new() -> Self {
        Self {
            timing_deg: 15.0,
            dwell_ms: 3.0,
            coils_ok: 4,
            total_coils: 4,
            misfire_count: 0,
            system_ok: true,
        }
    }

    pub fn all_coils_ok(&self) -> bool {
        self.coils_ok == self.total_coils
    }

    pub fn misfire_free(&self) -> bool {
        self.misfire_count == 0
    }

    pub fn timing_ok(&self) -> bool {
        self.timing_deg > 5.0 && self.timing_deg < 40.0
    }

    pub fn needs_service(&self) -> bool {
        !self.all_coils_ok() || self.misfire_count > 10
    }

    pub fn health_score(&self) -> f64 {
        if !self.system_ok {
            return 0.0;
        }
        if !self.all_coils_ok() {
            return 40.0;
        }
        if !self.misfire_free() {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_coils() {
        let i = IgnitionController::new();
        assert!(i.all_coils_ok());
    }

    #[test]
    fn test_no_misfire() {
        let i = IgnitionController::new();
        assert!(i.misfire_free());
    }

    #[test]
    fn test_timing_ok() {
        let i = IgnitionController::new();
        assert!(i.timing_ok());
    }

    #[test]
    fn test_no_service() {
        let i = IgnitionController::new();
        assert!(!i.needs_service());
    }

    #[test]
    fn test_coil_fail() {
        let mut i = IgnitionController::new();
        i.coils_ok = 3;
        assert!(!i.all_coils_ok());
    }

    #[test]
    fn test_health() {
        let i = IgnitionController::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
