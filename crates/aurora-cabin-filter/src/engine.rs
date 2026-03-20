/// Cabin filter: pollen, carbon, flow, replacement
/// Phase 833

#[derive(Debug, Clone)]
pub struct CabinFilter {
    pub pollen_ok: bool,
    pub carbon_ok: bool,
    pub flow_ok: bool,
    pub clean: bool,
    pub seal_ok: bool,
}

impl Default for CabinFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinFilter {
    pub fn new() -> Self {
        Self {
            pollen_ok: true,
            carbon_ok: true,
            flow_ok: true,
            clean: true,
            seal_ok: true,
        }
    }

    pub fn filtration_ok(&self) -> bool {
        self.pollen_ok && self.carbon_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.flow_ok && self.clean && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.filtration_ok() && self.performance_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.clean || !self.flow_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clean { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtration() {
        let c = CabinFilter::new();
        assert!(c.filtration_ok());
    }

    #[test]
    fn test_performance() {
        let c = CabinFilter::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CabinFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CabinFilter::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_clean() {
        let mut c = CabinFilter::new();
        c.clean = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CabinFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
