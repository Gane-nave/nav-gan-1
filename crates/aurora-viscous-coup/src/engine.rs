/// Viscous coupling: silicone fluid, haldex, torque transfer
/// Phase 472

#[derive(Debug, Clone)]
pub struct ViscousCoup {
    pub fluid_ok: bool,
    pub torque_transfer_ok: bool,
    pub seal_ok: bool,
    pub temp_c: f64,
    pub max_temp_c: f64,
}

impl Default for ViscousCoup {
    fn default() -> Self {
        Self::new()
    }
}

impl ViscousCoup {
    pub fn new() -> Self {
        Self {
            fluid_ok: true,
            torque_transfer_ok: true,
            seal_ok: true,
            temp_c: 60.0,
            max_temp_c: 120.0,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.fluid_ok && self.torque_transfer_ok && self.seal_ok && self.temp_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.fluid_ok || !self.seal_ok
    }

    pub fn coupling_effective(&self) -> bool {
        self.torque_transfer_ok && self.fluid_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fluid_ok {
            return 10.0;
        }
        if !self.seal_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let v = ViscousCoup::new();
        assert!(v.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let v = ViscousCoup::new();
        assert!(v.all_ok());
    }

    #[test]
    fn test_no_service() {
        let v = ViscousCoup::new();
        assert!(!v.needs_service());
    }

    #[test]
    fn test_coupling() {
        let v = ViscousCoup::new();
        assert!(v.coupling_effective());
    }

    #[test]
    fn test_bad_fluid() {
        let mut v = ViscousCoup::new();
        v.fluid_ok = false;
        assert!(v.needs_service());
    }

    #[test]
    fn test_health() {
        let v = ViscousCoup::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
