/// aurora-sim-network: sim network
/// Phase 2530

#[derive(Debug, Clone)]
pub struct SimNetwork {
    pub latency_ok: bool,
    pub loss_ok: bool,
    pub jitter_ok: bool,
    pub bandwidth_ok: bool,
    pub partition_ok: bool,
}

impl Default for SimNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl SimNetwork {
    pub fn new() -> Self {
        Self {
            latency_ok: true,
            loss_ok: true,
            jitter_ok: true,
            bandwidth_ok: true,
            partition_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.latency_ok && self.loss_ok && self.jitter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bandwidth_ok && self.partition_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.latency_ok || !self.loss_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latency_ok {
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
        let c = SimNetwork::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimNetwork::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimNetwork::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimNetwork::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimNetwork::new();
        c.latency_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimNetwork::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimNetwork::default();
        assert!(c.all_ok());
    }
}
