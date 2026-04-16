/// tie rod: connect, adjust, transmit, lock, check
/// Phase 1200

#[derive(Debug, Clone)]
pub struct TieRod {
    pub connect_ok: bool,
    pub adjust_ok: bool,
    pub transmit_ok: bool,
    pub lock_ok: bool,
    pub check_ok: bool,
}

impl Default for TieRod {
    fn default() -> Self {
        Self::new()
    }
}

impl TieRod {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            adjust_ok: true,
            transmit_ok: true,
            lock_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.adjust_ok && self.transmit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.lock_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.adjust_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = TieRod::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TieRod::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TieRod::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TieRod::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TieRod::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TieRod::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
