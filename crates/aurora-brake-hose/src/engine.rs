/// Brake hose: flexibility, crack, bulge, fitting
/// Phase 664

#[derive(Debug, Clone)]
pub struct BrakeHose {
    pub flexible: bool,
    pub cracked: bool,
    pub bulging: bool,
    pub fitting_ok: bool,
    pub leak_free: bool,
}

impl Default for BrakeHose {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeHose {
    pub fn new() -> Self {
        Self {
            flexible: true,
            cracked: false,
            bulging: false,
            fitting_ok: true,
            leak_free: true,
        }
    }

    pub fn condition_ok(&self) -> bool {
        self.flexible && !self.cracked && !self.bulging
    }

    pub fn connection_ok(&self) -> bool {
        self.fitting_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.condition_ok() && self.connection_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || self.bulging
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition() {
        let c = BrakeHose::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_connection() {
        let c = BrakeHose::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeHose::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeHose::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = BrakeHose::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeHose::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
