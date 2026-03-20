/// radiator: flow, exchange, fan, thermostat, check
/// Phase 1248

#[derive(Debug, Clone)]
pub struct Radiator {
    pub flow_ok: bool,
    pub exchange_ok: bool,
    pub fan_ok: bool,
    pub thermostat_ok: bool,
    pub check_ok: bool,
}

impl Default for Radiator {
    fn default() -> Self {
        Self::new()
    }
}

impl Radiator {
    pub fn new() -> Self {
        Self {
            flow_ok: true,
            exchange_ok: true,
            fan_ok: true,
            thermostat_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.flow_ok && self.exchange_ok && self.fan_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.thermostat_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.flow_ok || !self.exchange_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flow_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Radiator::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Radiator::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Radiator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Radiator::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Radiator::new();
        c.flow_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Radiator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
