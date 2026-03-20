/// convoy ctrl: join, follow, adjust, leave, alert
/// Phase 1324

#[derive(Debug, Clone)]
pub struct ConvoyCtrl {
    pub join_ok: bool,
    pub follow_ok: bool,
    pub adjust_ok: bool,
    pub leave_ok: bool,
    pub alert_ok: bool,
}

impl Default for ConvoyCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvoyCtrl {
    pub fn new() -> Self {
        Self {
            join_ok: true,
            follow_ok: true,
            adjust_ok: true,
            leave_ok: true,
            alert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.join_ok && self.follow_ok && self.adjust_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.leave_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.join_ok || !self.follow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.join_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ConvoyCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConvoyCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConvoyCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConvoyCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConvoyCtrl::new();
        c.join_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConvoyCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
