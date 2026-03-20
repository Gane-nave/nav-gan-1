/// evaporator: absorb, expand, drain, temp, check
/// Phase 1264

#[derive(Debug, Clone)]
pub struct Evaporator2 {
    pub absorb_ok: bool,
    pub expand_ok: bool,
    pub drain_ok: bool,
    pub temp_ok: bool,
    pub check_ok: bool,
}

impl Default for Evaporator2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaporator2 {
    pub fn new() -> Self {
        Self {
            absorb_ok: true,
            expand_ok: true,
            drain_ok: true,
            temp_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.absorb_ok && self.expand_ok && self.drain_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.temp_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.absorb_ok || !self.expand_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.absorb_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Evaporator2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Evaporator2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Evaporator2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Evaporator2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Evaporator2::new();
        c.absorb_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Evaporator2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
