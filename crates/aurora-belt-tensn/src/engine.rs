/// Belt tensioner: spring tension, damper, alignment, wear
/// Phase 322

#[derive(Debug, Clone)]
pub struct BeltTensioner {
    pub tension_n: f64,
    pub target_n: f64,
    pub damper_ok: bool,
    pub alignment_ok: bool,
    pub wear_pct: f64,
}

impl Default for BeltTensioner {
    fn default() -> Self {
        Self::new()
    }
}

impl BeltTensioner {
    pub fn new() -> Self {
        Self {
            tension_n: 400.0,
            target_n: 400.0,
            damper_ok: true,
            alignment_ok: true,
            wear_pct: 20.0,
        }
    }

    pub fn tension_ok(&self) -> bool {
        (self.tension_n - self.target_n).abs() < 50.0
    }

    pub fn all_ok(&self) -> bool {
        self.tension_ok() && self.damper_ok && self.alignment_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.wear_pct > 80.0 || !self.damper_ok
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.damper_ok {
            return 0.0;
        }
        if !self.alignment_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension() {
        let b = BeltTensioner::new();
        assert!(b.tension_ok());
    }

    #[test]
    fn test_all_ok() {
        let b = BeltTensioner::new();
        assert!(b.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let b = BeltTensioner::new();
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_life() {
        let b = BeltTensioner::new();
        assert!(b.remaining_life_pct() > 70.0);
    }

    #[test]
    fn test_worn() {
        let mut b = BeltTensioner::new();
        b.wear_pct = 90.0;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BeltTensioner::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
