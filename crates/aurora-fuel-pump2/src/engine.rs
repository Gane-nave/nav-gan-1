/// fuel pump: prime, deliver, pressure, filter, check
/// Phase 1258

#[derive(Debug, Clone)]
pub struct FuelPump2 {
    pub prime_ok: bool,
    pub deliver_ok: bool,
    pub pressure_ok: bool,
    pub filter_ok: bool,
    pub check_ok: bool,
}

impl Default for FuelPump2 {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelPump2 {
    pub fn new() -> Self {
        Self {
            prime_ok: true,
            deliver_ok: true,
            pressure_ok: true,
            filter_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.prime_ok && self.deliver_ok && self.pressure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.filter_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.prime_ok || !self.deliver_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.prime_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FuelPump2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuelPump2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelPump2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuelPump2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuelPump2::new();
        c.prime_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuelPump2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
