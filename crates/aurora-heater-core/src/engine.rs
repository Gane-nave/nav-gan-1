/// heater core: flow, exchange, valve, blend, check
/// Phase 1262

#[derive(Debug, Clone)]
pub struct HeaterCore {
    pub flow_ok: bool,
    pub exchange_ok: bool,
    pub valve_ok: bool,
    pub blend_ok: bool,
    pub check_ok: bool,
}

impl Default for HeaterCore {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaterCore {
    pub fn new() -> Self {
        Self {
            flow_ok: true,
            exchange_ok: true,
            valve_ok: true,
            blend_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.flow_ok && self.exchange_ok && self.valve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.blend_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.flow_ok || !self.exchange_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flow_ok {
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
        let c = HeaterCore::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HeaterCore::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeaterCore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HeaterCore::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HeaterCore::new();
        c.flow_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HeaterCore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
