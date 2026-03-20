/// Radiator: coolant flow, fin condition, pressure cap
/// Phase 510

#[derive(Debug, Clone)]
pub struct Radiator {
    pub coolant_temp_c: f64,
    pub max_temp_c: f64,
    pub flow_lpm: f64,
    pub cap_ok: bool,
    pub leak_free: bool,
}

impl Default for Radiator {
    fn default() -> Self {
        Self::new()
    }
}

impl Radiator {
    pub fn new() -> Self {
        Self {
            coolant_temp_c: 85.0,
            max_temp_c: 110.0,
            flow_lpm: 60.0,
            cap_ok: true,
            leak_free: true,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.coolant_temp_c < self.max_temp_c
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_lpm > 30.0
    }

    pub fn all_ok(&self) -> bool {
        self.temp_ok() && self.flow_ok() && self.cap_ok && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.cap_ok || !self.leak_free
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
    fn test_temp() {
        let c = Radiator::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_flow() {
        let c = Radiator::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Radiator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Radiator::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = Radiator::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Radiator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
