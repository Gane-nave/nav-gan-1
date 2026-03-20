/// headlamp ctrl: low, high, auto, adaptive, drl
/// Phase 1193

#[derive(Debug, Clone)]
pub struct HeadlampCtrl {
    pub low_ok: bool,
    pub high_ok: bool,
    pub auto_ok: bool,
    pub adaptive_ok: bool,
    pub drl_ok: bool,
}

impl Default for HeadlampCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlampCtrl {
    pub fn new() -> Self {
        Self {
            low_ok: true,
            high_ok: true,
            auto_ok: true,
            adaptive_ok: true,
            drl_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.low_ok && self.high_ok && self.auto_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adaptive_ok && self.drl_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.low_ok || !self.high_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.low_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = HeadlampCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HeadlampCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeadlampCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HeadlampCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HeadlampCtrl::new();
        c.low_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HeadlampCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
