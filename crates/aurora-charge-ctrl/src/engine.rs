/// charge ctrl: connect, negotiate, current, schedule, stop
/// Phase 1319

#[derive(Debug, Clone)]
pub struct ChargeCtrl {
    pub connect_ok: bool,
    pub negotiate_ok: bool,
    pub current_ok: bool,
    pub schedule_ok: bool,
    pub stop_ok: bool,
}

impl Default for ChargeCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeCtrl {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            negotiate_ok: true,
            current_ok: true,
            schedule_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.negotiate_ok && self.current_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.schedule_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.negotiate_ok
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
        let c = ChargeCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChargeCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargeCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChargeCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChargeCtrl::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChargeCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
