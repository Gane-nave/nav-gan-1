/// fleet fuel: monitor, consume, refill, optimize, log
/// Phase 1413

#[derive(Debug, Clone)]
pub struct FleetFuel {
    pub monitor_ok: bool,
    pub consume_ok: bool,
    pub refill_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetFuel {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetFuel {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            consume_ok: true,
            refill_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.consume_ok && self.refill_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.consume_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok {
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
        let c = FleetFuel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetFuel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetFuel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetFuel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetFuel::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetFuel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
