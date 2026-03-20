/// valve train: lift, duration, overlap, adjust, check
/// Phase 1234

#[derive(Debug, Clone)]
pub struct ValveTrain {
    pub lift_ok: bool,
    pub duration_ok: bool,
    pub overlap_ok: bool,
    pub adjust_ok: bool,
    pub check_ok: bool,
}

impl Default for ValveTrain {
    fn default() -> Self {
        Self::new()
    }
}

impl ValveTrain {
    pub fn new() -> Self {
        Self {
            lift_ok: true,
            duration_ok: true,
            overlap_ok: true,
            adjust_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lift_ok && self.duration_ok && self.overlap_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adjust_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lift_ok || !self.duration_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lift_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ValveTrain::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ValveTrain::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValveTrain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ValveTrain::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ValveTrain::new();
        c.lift_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ValveTrain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
