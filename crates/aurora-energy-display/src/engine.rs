/// Energy display: consumption, regen, trip, lifetime
/// Phase 880

#[derive(Debug, Clone)]
pub struct EnergyDisplay {
    pub consumption_ok: bool,
    pub regen_ok: bool,
    pub trip_ok: bool,
    pub lifetime_ok: bool,
    pub accuracy_ok: bool,
}

impl Default for EnergyDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyDisplay {
    pub fn new() -> Self {
        Self {
            consumption_ok: true,
            regen_ok: true,
            trip_ok: true,
            lifetime_ok: true,
            accuracy_ok: true,
        }
    }

    pub fn metrics_ok(&self) -> bool {
        self.consumption_ok && self.regen_ok
    }

    pub fn history_ok(&self) -> bool {
        self.trip_ok && self.lifetime_ok && self.accuracy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.metrics_ok() && self.history_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.accuracy_ok || !self.consumption_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.consumption_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let c = EnergyDisplay::new();
        assert!(c.metrics_ok());
    }

    #[test]
    fn test_history() {
        let c = EnergyDisplay::new();
        assert!(c.history_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EnergyDisplay::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = EnergyDisplay::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_accuracy() {
        let mut c = EnergyDisplay::new();
        c.accuracy_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = EnergyDisplay::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
