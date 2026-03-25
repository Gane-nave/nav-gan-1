/// Bump stop: jounce bumper, progressive rate, impact absorption
/// Phase 332

#[derive(Debug, Clone)]
pub struct BumpStop {
    pub compressed: bool,
    pub compression_pct: f64,
    pub material_ok: bool,
    pub cracked: bool,
    pub hardness_shore: f64,
}

impl Default for BumpStop {
    fn default() -> Self {
        Self::new()
    }
}

impl BumpStop {
    pub fn new() -> Self {
        Self {
            compressed: false,
            compression_pct: 0.0,
            material_ok: true,
            cracked: false,
            hardness_shore: 60.0,
        }
    }

    pub fn engaged(&self) -> bool {
        self.compression_pct > 5.0
    }

    pub fn fully_compressed(&self) -> bool {
        self.compression_pct > 90.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.material_ok
    }

    pub fn hardness_ok(&self) -> bool {
        self.hardness_shore > 40.0 && self.hardness_shore < 80.0
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        if !self.material_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_engaged() {
        let b = BumpStop::new();
        assert!(!b.engaged());
    }

    #[test]
    fn test_not_compressed() {
        let b = BumpStop::new();
        assert!(!b.fully_compressed());
    }

    #[test]
    fn test_no_replace() {
        let b = BumpStop::new();
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_hardness() {
        let b = BumpStop::new();
        assert!(b.hardness_ok());
    }

    #[test]
    fn test_cracked() {
        let mut b = BumpStop::new();
        b.cracked = true;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BumpStop::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
