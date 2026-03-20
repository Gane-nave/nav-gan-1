/// crank sensor: pulse, sync, position, speed, check
/// Phase 1245

#[derive(Debug, Clone)]
pub struct CrankSensor {
    pub pulse_ok: bool,
    pub sync_ok: bool,
    pub position_ok: bool,
    pub speed_ok: bool,
    pub check_ok: bool,
}

impl Default for CrankSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl CrankSensor {
    pub fn new() -> Self {
        Self {
            pulse_ok: true,
            sync_ok: true,
            position_ok: true,
            speed_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pulse_ok && self.sync_ok && self.position_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.speed_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pulse_ok || !self.sync_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pulse_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CrankSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CrankSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrankSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CrankSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CrankSensor::new();
        c.pulse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CrankSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
