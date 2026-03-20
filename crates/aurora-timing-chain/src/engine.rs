/// Timing chain: stretch monitoring, tensioner, guide rail wear
/// Phase 313

#[derive(Debug, Clone)]
pub struct TimingChain {
    pub stretch_mm: f64,
    pub max_stretch_mm: f64,
    pub tensioner_ok: bool,
    pub guide_ok: bool,
    pub mileage_km: f64,
}

impl Default for TimingChain {
    fn default() -> Self {
        Self::new()
    }
}

impl TimingChain {
    pub fn new() -> Self {
        Self {
            stretch_mm: 0.5,
            max_stretch_mm: 3.0,
            tensioner_ok: true,
            guide_ok: true,
            mileage_km: 50000.0,
        }
    }

    pub fn stretch_ok(&self) -> bool {
        self.stretch_mm < self.max_stretch_mm * 0.8
    }

    pub fn needs_replacement(&self) -> bool {
        self.stretch_mm >= self.max_stretch_mm || !self.guide_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tensioner_ok && self.guide_ok && self.stretch_ok()
    }

    pub fn remaining_life_pct(&self) -> f64 {
        ((self.max_stretch_mm - self.stretch_mm) / self.max_stretch_mm * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_replacement() {
            return 0.0;
        }
        if !self.tensioner_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stretch_ok() {
        let t = TimingChain::new();
        assert!(t.stretch_ok());
    }

    #[test]
    fn test_no_replace() {
        let t = TimingChain::new();
        assert!(!t.needs_replacement());
    }

    #[test]
    fn test_all_ok() {
        let t = TimingChain::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_life() {
        let t = TimingChain::new();
        assert!(t.remaining_life_pct() > 80.0);
    }

    #[test]
    fn test_worn() {
        let mut t = TimingChain::new();
        t.stretch_mm = 3.5;
        assert!(t.needs_replacement());
    }

    #[test]
    fn test_health() {
        let t = TimingChain::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
