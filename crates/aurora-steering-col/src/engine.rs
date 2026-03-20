/// steering col: tilt, telescope, lock, collapse, adjust
/// Phase 1207

#[derive(Debug, Clone)]
pub struct SteeringCol {
    pub tilt_ok: bool,
    pub telescope_ok: bool,
    pub lock_ok: bool,
    pub collapse_ok: bool,
    pub adjust_ok: bool,
}

impl Default for SteeringCol {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringCol {
    pub fn new() -> Self {
        Self {
            tilt_ok: true,
            telescope_ok: true,
            lock_ok: true,
            collapse_ok: true,
            adjust_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tilt_ok && self.telescope_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.collapse_ok && self.adjust_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tilt_ok || !self.telescope_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tilt_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SteeringCol::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SteeringCol::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringCol::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SteeringCol::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SteeringCol::new();
        c.tilt_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SteeringCol::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
