/// brake pad: press, wear, indicate, bed, replace
/// Phase 1205

#[derive(Debug, Clone)]
pub struct BrakePad {
    pub press_ok: bool,
    pub wear_ok: bool,
    pub indicate_ok: bool,
    pub bed_ok: bool,
    pub replace_ok: bool,
}

impl Default for BrakePad {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakePad {
    pub fn new() -> Self {
        Self {
            press_ok: true,
            wear_ok: true,
            indicate_ok: true,
            bed_ok: true,
            replace_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.press_ok && self.wear_ok && self.indicate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bed_ok && self.replace_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.press_ok || !self.wear_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.press_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BrakePad::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BrakePad::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakePad::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BrakePad::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BrakePad::new();
        c.press_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BrakePad::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
