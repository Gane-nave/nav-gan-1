/// 5G modem: connect, handover, slice, latency, throughput
/// Phase 980

#[derive(Debug, Clone)]
pub struct FiveGModem {
    pub connect_ok: bool,
    pub handover_ok: bool,
    pub slice_ok: bool,
    pub latency_ok: bool,
    pub throughput_ok: bool,
}

impl Default for FiveGModem {
    fn default() -> Self {
        Self::new()
    }
}

impl FiveGModem {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            handover_ok: true,
            slice_ok: true,
            latency_ok: true,
            throughput_ok: true,
        }
    }

    pub fn connectivity_ok(&self) -> bool {
        self.connect_ok && self.handover_ok && self.slice_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.latency_ok && self.throughput_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connectivity_ok() && self.performance_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.connect_ok || !self.slice_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity() {
        let c = FiveGModem::new();
        assert!(c.connectivity_ok());
    }

    #[test]
    fn test_performance() {
        let c = FiveGModem::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FiveGModem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = FiveGModem::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_connect() {
        let mut c = FiveGModem::new();
        c.connect_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = FiveGModem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
