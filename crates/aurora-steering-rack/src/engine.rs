/// steering rack: translate, assist, center, ratio, check
/// Phase 1206

#[derive(Debug, Clone)]
pub struct SteeringRack {
    pub translate_ok: bool,
    pub assist_ok: bool,
    pub center_ok: bool,
    pub ratio_ok: bool,
    pub check_ok: bool,
}

impl Default for SteeringRack {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringRack {
    pub fn new() -> Self {
        Self {
            translate_ok: true,
            assist_ok: true,
            center_ok: true,
            ratio_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.translate_ok && self.assist_ok && self.center_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.ratio_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.translate_ok || !self.assist_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.translate_ok {
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
        let c = SteeringRack::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SteeringRack::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SteeringRack::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SteeringRack::new();
        c.translate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SteeringRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
