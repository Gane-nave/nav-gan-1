/// FlexRay bus: deterministic communication, fault-tolerant, dual channel
/// Phase 437

#[derive(Debug, Clone)]
pub struct FlexrayBus {
    pub channel_a_ok: bool,
    pub channel_b_ok: bool,
    pub sync_ok: bool,
    pub error_rate_ppm: f64,
    pub max_error_ppm: f64,
}

impl Default for FlexrayBus {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexrayBus {
    pub fn new() -> Self {
        Self {
            channel_a_ok: true,
            channel_b_ok: true,
            sync_ok: true,
            error_rate_ppm: 0.5,
            max_error_ppm: 10.0,
        }
    }

    pub fn dual_channel(&self) -> bool {
        self.channel_a_ok && self.channel_b_ok
    }

    pub fn fault_tolerant(&self) -> bool {
        self.channel_a_ok || self.channel_b_ok
    }

    pub fn error_ok(&self) -> bool {
        self.error_rate_ppm < self.max_error_ppm
    }

    pub fn needs_service(&self) -> bool {
        !self.fault_tolerant() || !self.sync_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fault_tolerant() {
            return 0.0;
        }
        if !self.dual_channel() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual() {
        let f = FlexrayBus::new();
        assert!(f.dual_channel());
    }

    #[test]
    fn test_tolerant() {
        let f = FlexrayBus::new();
        assert!(f.fault_tolerant());
    }

    #[test]
    fn test_error() {
        let f = FlexrayBus::new();
        assert!(f.error_ok());
    }

    #[test]
    fn test_no_service() {
        let f = FlexrayBus::new();
        assert!(!f.needs_service());
    }

    #[test]
    fn test_single_channel() {
        let mut f = FlexrayBus::new();
        f.channel_a_ok = false;
        assert!(!f.dual_channel());
        assert!(f.fault_tolerant());
    }

    #[test]
    fn test_health() {
        let f = FlexrayBus::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
