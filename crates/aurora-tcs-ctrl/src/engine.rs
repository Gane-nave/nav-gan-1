/// tcs ctrl: slip, reduce, grip, adapt, report
/// Phase 1160

#[derive(Debug, Clone)]
pub struct TcsCtrl {
    pub slip_ok: bool,
    pub reduce_ok: bool,
    pub grip_ok: bool,
    pub adapt_ok: bool,
    pub report_ok: bool,
}

impl Default for TcsCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl TcsCtrl {
    pub fn new() -> Self {
        Self {
            slip_ok: true,
            reduce_ok: true,
            grip_ok: true,
            adapt_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.slip_ok && self.reduce_ok && self.grip_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.slip_ok || !self.reduce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.slip_ok {
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
        let c = TcsCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TcsCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TcsCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TcsCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TcsCtrl::new();
        c.slip_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TcsCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
