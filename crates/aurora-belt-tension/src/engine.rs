/// Belt tension monitoring: serpentine belt, timing belt, tensioner health
/// Phase 171

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeltType {
    Serpentine,
    Timing,
    Alternator,
    AC,
}

#[derive(Debug, Clone)]
pub struct BeltMonitor {
    pub belt_type: BeltType,
    pub tension_n: f64,
    pub target_tension_n: f64,
    pub wear_pct: f64,
    pub km_since_replace: f64,
    pub max_km: f64,
}

impl BeltMonitor {
    pub fn new(belt_type: BeltType) -> Self {
        Self {
            belt_type,
            tension_n: 500.0,
            target_tension_n: 500.0,
            wear_pct: 0.0,
            km_since_replace: 0.0,
            max_km: 100000.0,
        }
    }

    pub fn tension_ok(&self) -> bool {
        let deviation = (self.tension_n - self.target_tension_n).abs() / self.target_tension_n;
        deviation < 0.15
    }

    pub fn needs_replacement(&self) -> bool {
        self.wear_pct > 80.0 || self.km_since_replace > self.max_km
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.wear_pct).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        let tension_s = if self.tension_ok() { 50.0 } else { 20.0 };
        let wear_s = self.remaining_life_pct() / 100.0 * 50.0;
        tension_s + wear_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension_ok() {
        let b = BeltMonitor::new(BeltType::Serpentine);
        assert!(b.tension_ok());
    }

    #[test]
    fn test_tension_bad() {
        let mut b = BeltMonitor::new(BeltType::Timing);
        b.tension_n = 300.0;
        assert!(!b.tension_ok());
    }

    #[test]
    fn test_no_replacement() {
        let b = BeltMonitor::new(BeltType::AC);
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_needs_replacement() {
        let mut b = BeltMonitor::new(BeltType::Serpentine);
        b.wear_pct = 90.0;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BeltMonitor::new(BeltType::Alternator);
        assert!(b.health_score() > 90.0);
    }

    #[test]
    fn test_remaining_life() {
        let mut b = BeltMonitor::new(BeltType::Timing);
        b.wear_pct = 40.0;
        assert!((b.remaining_life_pct() - 60.0).abs() < 0.1);
    }
}
