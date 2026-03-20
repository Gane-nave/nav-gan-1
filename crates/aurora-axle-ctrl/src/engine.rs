/// axle ctrl: drive, float, lock, ratio, check
/// Phase 1220

#[derive(Debug, Clone)]
pub struct AxleCtrl {
    pub drive_ok: bool,
    pub float_ok: bool,
    pub lock_ok: bool,
    pub ratio_ok: bool,
    pub check_ok: bool,
}

impl Default for AxleCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl AxleCtrl {
    pub fn new() -> Self {
        Self {
            drive_ok: true,
            float_ok: true,
            lock_ok: true,
            ratio_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.drive_ok && self.float_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.ratio_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.drive_ok || !self.float_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.drive_ok {
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
        let c = AxleCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AxleCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AxleCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AxleCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AxleCtrl::new();
        c.drive_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AxleCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
