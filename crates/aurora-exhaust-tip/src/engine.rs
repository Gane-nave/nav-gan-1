/// Exhaust tip: chrome, welds, alignment, heat color
/// Phase 613

#[derive(Debug, Clone)]
pub struct ExhaustTip {
    pub chrome_ok: bool,
    pub welds_ok: bool,
    pub aligned: bool,
    pub heat_color_ok: bool,
    pub clamp_ok: bool,
}

impl Default for ExhaustTip {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustTip {
    pub fn new() -> Self {
        Self {
            chrome_ok: true,
            welds_ok: true,
            aligned: true,
            heat_color_ok: true,
            clamp_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.chrome_ok && self.heat_color_ok
    }

    pub fn mounting_ok(&self) -> bool {
        self.welds_ok && self.aligned && self.clamp_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.mounting_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.welds_ok || !self.chrome_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.welds_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appearance() {
        let c = ExhaustTip::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_mounting() {
        let c = ExhaustTip::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ExhaustTip::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ExhaustTip::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_welds() {
        let mut c = ExhaustTip::new();
        c.welds_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ExhaustTip::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
