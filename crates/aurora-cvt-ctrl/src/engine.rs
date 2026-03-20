/// cvt ctrl: ratio, belt, pulley, clamp, report
/// Phase 1214

#[derive(Debug, Clone)]
pub struct CvtCtrl {
    pub ratio_ok: bool,
    pub belt_ok: bool,
    pub pulley_ok: bool,
    pub clamp_ok: bool,
    pub report_ok: bool,
}

impl Default for CvtCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl CvtCtrl {
    pub fn new() -> Self {
        Self {
            ratio_ok: true,
            belt_ok: true,
            pulley_ok: true,
            clamp_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ratio_ok && self.belt_ok && self.pulley_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clamp_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ratio_ok || !self.belt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ratio_ok {
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
        let c = CvtCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CvtCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CvtCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CvtCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CvtCtrl::new();
        c.ratio_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CvtCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
