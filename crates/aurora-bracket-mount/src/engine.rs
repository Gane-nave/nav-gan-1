/// Bracket mount: structural bracket, vibration isolator, load path
/// Phase 408

#[derive(Debug, Clone)]
pub struct BracketMount {
    pub load_kn: f64,
    pub max_load_kn: f64,
    pub cracked: bool,
    pub bolts_ok: bool,
    pub isolator_ok: bool,
}

impl Default for BracketMount {
    fn default() -> Self {
        Self::new()
    }
}

impl BracketMount {
    pub fn new() -> Self {
        Self {
            load_kn: 3.0,
            max_load_kn: 10.0,
            cracked: false,
            bolts_ok: true,
            isolator_ok: true,
        }
    }

    pub fn load_ok(&self) -> bool {
        self.load_kn <= self.max_load_kn
    }

    pub fn structural_ok(&self) -> bool {
        !self.cracked && self.bolts_ok
    }

    pub fn all_ok(&self) -> bool {
        self.load_ok() && self.structural_ok() && self.isolator_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 0.0;
        }
        if !self.bolts_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let b = BracketMount::new();
        assert!(b.load_ok());
    }

    #[test]
    fn test_structural() {
        let b = BracketMount::new();
        assert!(b.structural_ok());
    }

    #[test]
    fn test_all_ok() {
        let b = BracketMount::new();
        assert!(b.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let b = BracketMount::new();
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_cracked() {
        let mut b = BracketMount::new();
        b.cracked = true;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BracketMount::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
