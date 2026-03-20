/// PCV valve: positive crankcase ventilation, flow control
/// Phase 507

#[derive(Debug, Clone)]
pub struct PcvValve {
    pub flow_lpm: f64,
    pub min_flow_lpm: f64,
    pub stuck: bool,
    pub clogged: bool,
    pub grommet_ok: bool,
}

impl Default for PcvValve {
    fn default() -> Self {
        Self::new()
    }
}

impl PcvValve {
    pub fn new() -> Self {
        Self {
            flow_lpm: 25.0,
            min_flow_lpm: 10.0,
            stuck: false,
            clogged: false,
            grommet_ok: true,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_lpm > self.min_flow_lpm
    }

    pub fn is_functional(&self) -> bool {
        !self.stuck && !self.clogged
    }

    pub fn all_ok(&self) -> bool {
        self.flow_ok() && self.is_functional() && self.grommet_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.stuck || self.clogged
    }

    pub fn health_score(&self) -> f64 {
        if self.stuck { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        let c = PcvValve::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_functional() {
        let c = PcvValve::new();
        assert!(c.is_functional());
    }

    #[test]
    fn test_all_ok() {
        let c = PcvValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = PcvValve::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_stuck() {
        let mut c = PcvValve::new();
        c.stuck = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = PcvValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
