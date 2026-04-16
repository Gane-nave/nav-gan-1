/// Transfer case: 4WD/AWD, chain drive, gear reduction, fluid level
/// Phase 463

#[derive(Debug, Clone)]
pub struct TransferCase {
    pub fluid_ok: bool,
    pub chain_ok: bool,
    pub encoder_ok: bool,
    pub motor_ok: bool,
    pub mode: u8,
}

impl Default for TransferCase {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferCase {
    pub fn new() -> Self {
        Self {
            fluid_ok: true,
            chain_ok: true,
            encoder_ok: true,
            motor_ok: true,
            mode: 1,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.fluid_ok && self.chain_ok && self.encoder_ok && self.motor_ok
    }

    pub fn shifting_ok(&self) -> bool {
        self.motor_ok && self.encoder_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.fluid_ok || !self.chain_ok
    }

    pub fn four_wd_capable(&self) -> bool {
        self.all_ok()
    }

    pub fn health_score(&self) -> f64 {
        if !self.chain_ok {
            return 10.0;
        }
        if !self.fluid_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let t = TransferCase::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_shifting() {
        let t = TransferCase::new();
        assert!(t.shifting_ok());
    }

    #[test]
    fn test_no_service() {
        let t = TransferCase::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_4wd() {
        let t = TransferCase::new();
        assert!(t.four_wd_capable());
    }

    #[test]
    fn test_bad_chain() {
        let mut t = TransferCase::new();
        t.chain_ok = false;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TransferCase::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
