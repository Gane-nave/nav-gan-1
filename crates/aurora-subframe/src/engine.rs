/// Subframe: cradle mount, bushing condition, alignment
/// Phase 333

#[derive(Debug, Clone)]
pub struct Subframe {
    pub bushings_ok: bool,
    pub mounts_ok: bool,
    pub alignment_ok: bool,
    pub corrosion_pct: f64,
    pub cracked: bool,
}

impl Default for Subframe {
    fn default() -> Self {
        Self::new()
    }
}

impl Subframe {
    pub fn new() -> Self {
        Self {
            bushings_ok: true,
            mounts_ok: true,
            alignment_ok: true,
            corrosion_pct: 5.0,
            cracked: false,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.bushings_ok && self.mounts_ok && self.alignment_ok && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || self.corrosion_pct > 50.0
    }

    pub fn needs_bushings(&self) -> bool {
        !self.bushings_ok
    }

    pub fn structural_ok(&self) -> bool {
        !self.cracked && self.corrosion_pct < 30.0
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        if !self.mounts_ok {
            return 30.0;
        }
        if !self.bushings_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let s = Subframe::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let s = Subframe::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_no_bushings() {
        let s = Subframe::new();
        assert!(!s.needs_bushings());
    }

    #[test]
    fn test_structural() {
        let s = Subframe::new();
        assert!(s.structural_ok());
    }

    #[test]
    fn test_cracked() {
        let mut s = Subframe::new();
        s.cracked = true;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = Subframe::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
