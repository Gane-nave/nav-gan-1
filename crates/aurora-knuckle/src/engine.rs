/// Steering knuckle: spindle, mount, ABS bracket
/// Phase 651

#[derive(Debug, Clone)]
pub struct Knuckle {
    pub spindle_ok: bool,
    pub mount_ok: bool,
    pub abs_bracket_ok: bool,
    pub cracked: bool,
    pub corrosion_free: bool,
}

impl Default for Knuckle {
    fn default() -> Self {
        Self::new()
    }
}

impl Knuckle {
    pub fn new() -> Self {
        Self {
            spindle_ok: true,
            mount_ok: true,
            abs_bracket_ok: true,
            cracked: false,
            corrosion_free: true,
        }
    }

    pub fn structural_ok(&self) -> bool {
        self.spindle_ok && !self.cracked && self.corrosion_free
    }

    pub fn mounting_ok(&self) -> bool {
        self.mount_ok && self.abs_bracket_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structural_ok() && self.mounting_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.spindle_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structural() {
        let c = Knuckle::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_mounting() {
        let c = Knuckle::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Knuckle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Knuckle::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = Knuckle::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Knuckle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
