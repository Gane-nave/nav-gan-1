/// AC condenser: fins, subcooling, fan, debris
/// Phase 623

#[derive(Debug, Clone)]
pub struct Condenser {
    pub fins_ok: bool,
    pub subcooling_ok: bool,
    pub fan_ok: bool,
    pub debris_free: bool,
    pub leak_free: bool,
}

impl Default for Condenser {
    fn default() -> Self {
        Self::new()
    }
}

impl Condenser {
    pub fn new() -> Self {
        Self {
            fins_ok: true,
            subcooling_ok: true,
            fan_ok: true,
            debris_free: true,
            leak_free: true,
        }
    }

    pub fn cooling_ok(&self) -> bool {
        self.fins_ok && self.subcooling_ok
    }

    pub fn airflow_ok(&self) -> bool {
        self.fan_ok && self.debris_free
    }

    pub fn all_ok(&self) -> bool {
        self.cooling_ok() && self.airflow_ok() && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.fins_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling() {
        let c = Condenser::new();
        assert!(c.cooling_ok());
    }

    #[test]
    fn test_airflow() {
        let c = Condenser::new();
        assert!(c.airflow_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Condenser::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Condenser::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = Condenser::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Condenser::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
