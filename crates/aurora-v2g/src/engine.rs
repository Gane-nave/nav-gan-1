/// Vehicle-to-grid: bidirectional, islanding, metering
/// Phase 856

#[derive(Debug, Clone)]
pub struct V2g {
    pub bidir_ok: bool,
    pub island_ok: bool,
    pub meter_ok: bool,
    pub grid_ok: bool,
    pub safety_ok: bool,
}

impl Default for V2g {
    fn default() -> Self {
        Self::new()
    }
}

impl V2g {
    pub fn new() -> Self {
        Self {
            bidir_ok: true,
            island_ok: true,
            meter_ok: true,
            grid_ok: true,
            safety_ok: true,
        }
    }

    pub fn power_ok(&self) -> bool {
        self.bidir_ok && self.grid_ok && self.safety_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.island_ok && self.meter_ok
    }

    pub fn all_ok(&self) -> bool {
        self.power_ok() && self.monitoring_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bidir_ok || !self.grid_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bidir_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let c = V2g::new();
        assert!(c.power_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = V2g::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2g::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = V2g::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bidir() {
        let mut c = V2g::new();
        c.bidir_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = V2g::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
