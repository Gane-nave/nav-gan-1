/// oil filter: trap, bypass, indicate, flow, check
/// Phase 1252

#[derive(Debug, Clone)]
pub struct OilFilter2 {
    pub trap_ok: bool,
    pub bypass_ok: bool,
    pub indicate_ok: bool,
    pub flow_ok: bool,
    pub check_ok: bool,
}

impl Default for OilFilter2 {
    fn default() -> Self {
        Self::new()
    }
}

impl OilFilter2 {
    pub fn new() -> Self {
        Self {
            trap_ok: true,
            bypass_ok: true,
            indicate_ok: true,
            flow_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.trap_ok && self.bypass_ok && self.indicate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flow_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.trap_ok || !self.bypass_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trap_ok {
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
        let c = OilFilter2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OilFilter2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilFilter2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OilFilter2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OilFilter2::new();
        c.trap_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OilFilter2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
