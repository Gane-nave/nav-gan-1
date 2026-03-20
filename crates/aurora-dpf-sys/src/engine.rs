/// dpf sys: trap, regenerate, sense, clean, check
/// Phase 1240

#[derive(Debug, Clone)]
pub struct DpfSys {
    pub trap_ok: bool,
    pub regenerate_ok: bool,
    pub sense_ok: bool,
    pub clean_ok: bool,
    pub check_ok: bool,
}

impl Default for DpfSys {
    fn default() -> Self {
        Self::new()
    }
}

impl DpfSys {
    pub fn new() -> Self {
        Self {
            trap_ok: true,
            regenerate_ok: true,
            sense_ok: true,
            clean_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.trap_ok && self.regenerate_ok && self.sense_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clean_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.trap_ok || !self.regenerate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trap_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DpfSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DpfSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DpfSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DpfSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DpfSys::new();
        c.trap_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DpfSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
