/// Diesel particulate filter: soot loading, regeneration, backpressure
/// Phase 304

#[derive(Debug, Clone)]
pub struct DpfFilter {
    pub soot_loading_pct: f64,
    pub regen_active: bool,
    pub backpressure_kpa: f64,
    pub max_backpressure_kpa: f64,
    pub regen_count: u32,
    pub filter_ok: bool,
}

impl Default for DpfFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DpfFilter {
    pub fn new() -> Self {
        Self {
            soot_loading_pct: 30.0,
            regen_active: false,
            backpressure_kpa: 5.0,
            max_backpressure_kpa: 20.0,
            regen_count: 50,
            filter_ok: true,
        }
    }

    pub fn needs_regen(&self) -> bool {
        self.soot_loading_pct > 80.0
    }

    pub fn backpressure_ok(&self) -> bool {
        self.backpressure_kpa < self.max_backpressure_kpa
    }

    pub fn blocked(&self) -> bool {
        self.soot_loading_pct > 95.0 || self.backpressure_kpa > self.max_backpressure_kpa
    }

    pub fn needs_replacement(&self) -> bool {
        !self.filter_ok || self.regen_count > 500
    }

    pub fn health_score(&self) -> f64 {
        if !self.filter_ok {
            return 0.0;
        }
        if self.blocked() {
            return 10.0;
        }
        if self.needs_regen() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_regen() {
        let d = DpfFilter::new();
        assert!(!d.needs_regen());
    }

    #[test]
    fn test_bp_ok() {
        let d = DpfFilter::new();
        assert!(d.backpressure_ok());
    }

    #[test]
    fn test_not_blocked() {
        let d = DpfFilter::new();
        assert!(!d.blocked());
    }

    #[test]
    fn test_no_replace() {
        let d = DpfFilter::new();
        assert!(!d.needs_replacement());
    }

    #[test]
    fn test_high_soot() {
        let mut d = DpfFilter::new();
        d.soot_loading_pct = 90.0;
        assert!(d.needs_regen());
    }

    #[test]
    fn test_health() {
        let d = DpfFilter::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
