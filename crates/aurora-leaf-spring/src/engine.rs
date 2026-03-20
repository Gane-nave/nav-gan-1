/// Leaf spring: multi-leaf pack, deflection, fatigue
/// Phase 475

#[derive(Debug, Clone)]
pub struct LeafSpring {
    pub leaf_count: u32,
    pub deflection_mm: f64,
    pub max_deflection_mm: f64,
    pub cracked: bool,
    pub corroded: bool,
}

impl Default for LeafSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl LeafSpring {
    pub fn new() -> Self {
        Self {
            leaf_count: 5,
            deflection_mm: 30.0,
            max_deflection_mm: 80.0,
            cracked: false,
            corroded: false,
        }
    }

    pub fn deflection_pct(&self) -> f64 {
        (self.deflection_mm / self.max_deflection_mm) * 100.0
    }

    pub fn over_deflected(&self) -> bool {
        self.deflection_mm > self.max_deflection_mm * 0.9
    }

    pub fn all_ok(&self) -> bool {
        !self.cracked && !self.corroded && !self.over_deflected()
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deflection() {
        let c = LeafSpring::new();
        assert!(c.deflection_pct() < 50.0);
    }

    #[test]
    fn test_not_over() {
        let c = LeafSpring::new();
        assert!(!c.over_deflected());
    }

    #[test]
    fn test_all_ok() {
        let c = LeafSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = LeafSpring::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cracked() {
        let mut c = LeafSpring::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = LeafSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
