/// Synchro ring: synchronizer, brass/carbon, friction cone
/// Phase 467

#[derive(Debug, Clone)]
pub struct SynchroRing {
    pub friction_ok: bool,
    pub teeth_ok: bool,
    pub cone_ok: bool,
    pub wear_pct: f64,
    pub shift_smooth: bool,
}

impl Default for SynchroRing {
    fn default() -> Self {
        Self::new()
    }
}

impl SynchroRing {
    pub fn new() -> Self {
        Self {
            friction_ok: true,
            teeth_ok: true,
            cone_ok: true,
            wear_pct: 15.0,
            shift_smooth: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.friction_ok && self.teeth_ok && self.cone_ok && self.shift_smooth
    }

    pub fn needs_replacement(&self) -> bool {
        self.wear_pct > 80.0 || !self.teeth_ok
    }

    pub fn shift_quality_ok(&self) -> bool {
        self.shift_smooth && self.friction_ok
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.teeth_ok {
            return 0.0;
        }
        if !self.friction_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let s = SynchroRing::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let s = SynchroRing::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_quality() {
        let s = SynchroRing::new();
        assert!(s.shift_quality_ok());
    }

    #[test]
    fn test_life() {
        let s = SynchroRing::new();
        assert!(s.remaining_life_pct() > 80.0);
    }

    #[test]
    fn test_worn() {
        let mut s = SynchroRing::new();
        s.wear_pct = 90.0;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = SynchroRing::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
