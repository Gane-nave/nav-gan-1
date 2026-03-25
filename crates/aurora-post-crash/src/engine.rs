/// Post-crash: fuel cutoff, unlock, hazard, eCall
/// Phase 842

#[derive(Debug, Clone)]
pub struct PostCrash {
    pub fuel_cutoff_ok: bool,
    pub unlock_ok: bool,
    pub hazard_ok: bool,
    pub ecall_ok: bool,
    pub battery_ok: bool,
}

impl Default for PostCrash {
    fn default() -> Self {
        Self::new()
    }
}

impl PostCrash {
    pub fn new() -> Self {
        Self {
            fuel_cutoff_ok: true,
            unlock_ok: true,
            hazard_ok: true,
            ecall_ok: true,
            battery_ok: true,
        }
    }

    pub fn safety_ok(&self) -> bool {
        self.fuel_cutoff_ok && self.unlock_ok && self.battery_ok
    }

    pub fn communication_ok(&self) -> bool {
        self.hazard_ok && self.ecall_ok
    }

    pub fn all_ok(&self) -> bool {
        self.safety_ok() && self.communication_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.ecall_ok || !self.fuel_cutoff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fuel_cutoff_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety() {
        let c = PostCrash::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_communication() {
        let c = PostCrash::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PostCrash::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PostCrash::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ecall() {
        let mut c = PostCrash::new();
        c.ecall_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PostCrash::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
