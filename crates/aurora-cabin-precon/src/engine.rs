/// Cabin preconditioning: schedule, temp target, ventilation
/// Phase 876

#[derive(Debug, Clone)]
pub struct CabinPrecon {
    pub schedule_ok: bool,
    pub target_ok: bool,
    pub vent_ok: bool,
    pub battery_ok: bool,
    pub remote_ok: bool,
}

impl Default for CabinPrecon {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinPrecon {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            target_ok: true,
            vent_ok: true,
            battery_ok: true,
            remote_ok: true,
        }
    }

    pub fn climate_ok(&self) -> bool {
        self.schedule_ok && self.target_ok && self.vent_ok
    }

    pub fn control_ok(&self) -> bool {
        self.battery_ok && self.remote_ok
    }

    pub fn all_ok(&self) -> bool {
        self.climate_ok() && self.control_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.schedule_ok || !self.target_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climate() {
        let c = CabinPrecon::new();
        assert!(c.climate_ok());
    }

    #[test]
    fn test_control() {
        let c = CabinPrecon::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CabinPrecon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = CabinPrecon::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_schedule() {
        let mut c = CabinPrecon::new();
        c.schedule_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = CabinPrecon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
