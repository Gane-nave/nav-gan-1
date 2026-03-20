/// lka sys: detect, center, correct, hold, disengage
/// Phase 1167

#[derive(Debug, Clone)]
pub struct LkaSys {
    pub detect_ok: bool,
    pub center_ok: bool,
    pub correct_ok: bool,
    pub hold_ok: bool,
    pub disengage_ok: bool,
}

impl Default for LkaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl LkaSys {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            center_ok: true,
            correct_ok: true,
            hold_ok: true,
            disengage_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.center_ok && self.correct_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.hold_ok && self.disengage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.center_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = LkaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LkaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LkaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LkaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LkaSys::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LkaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
