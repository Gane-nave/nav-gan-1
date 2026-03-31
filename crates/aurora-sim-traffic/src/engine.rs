/// aurora-sim-traffic: sim traffic
/// Phase 2525

#[derive(Debug, Clone)]
pub struct SimTraffic {
    pub flow_ok: bool,
    pub density_ok: bool,
    pub incident_ok: bool,
    pub signal_ok: bool,
    pub merge_ok: bool,
}

impl Default for SimTraffic {
    fn default() -> Self {
        Self::new()
    }
}

impl SimTraffic {
    pub fn new() -> Self {
        Self {
            flow_ok: true,
            density_ok: true,
            incident_ok: true,
            signal_ok: true,
            merge_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.flow_ok && self.density_ok && self.incident_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.signal_ok && self.merge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.flow_ok || !self.density_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flow_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SimTraffic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimTraffic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimTraffic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimTraffic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimTraffic::new();
        c.flow_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimTraffic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimTraffic::default();
        assert!(c.all_ok());
    }
}
