/// Charge timer: schedule, rate, departure, priority
/// Phase 878

#[derive(Debug, Clone)]
pub struct ChargeTimer {
    pub schedule_ok: bool,
    pub rate_ok: bool,
    pub departure_ok: bool,
    pub priority_ok: bool,
    pub grid_ok: bool,
}

impl Default for ChargeTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeTimer {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            rate_ok: true,
            departure_ok: true,
            priority_ok: true,
            grid_ok: true,
        }
    }

    pub fn timing_ok(&self) -> bool {
        self.schedule_ok && self.departure_ok && self.priority_ok
    }

    pub fn power_ok(&self) -> bool {
        self.rate_ok && self.grid_ok
    }

    pub fn all_ok(&self) -> bool {
        self.timing_ok() && self.power_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.schedule_ok || !self.departure_ok
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
    fn test_timing() {
        let c = ChargeTimer::new();
        assert!(c.timing_ok());
    }

    #[test]
    fn test_power() {
        let c = ChargeTimer::new();
        assert!(c.power_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargeTimer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = ChargeTimer::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_schedule() {
        let mut c = ChargeTimer::new();
        c.schedule_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = ChargeTimer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
