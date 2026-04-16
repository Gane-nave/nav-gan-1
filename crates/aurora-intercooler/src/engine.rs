/// intercooler: cool, flow, pressure, condense, check
/// Phase 1237

#[derive(Debug, Clone)]
pub struct Intercooler {
    pub cool_ok: bool,
    pub flow_ok: bool,
    pub pressure_ok: bool,
    pub condense_ok: bool,
    pub check_ok: bool,
}

impl Default for Intercooler {
    fn default() -> Self {
        Self::new()
    }
}

impl Intercooler {
    pub fn new() -> Self {
        Self {
            cool_ok: true,
            flow_ok: true,
            pressure_ok: true,
            condense_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.cool_ok && self.flow_ok && self.pressure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.condense_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.cool_ok || !self.flow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cool_ok {
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
        let c = Intercooler::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Intercooler::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Intercooler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Intercooler::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Intercooler::new();
        c.cool_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Intercooler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
