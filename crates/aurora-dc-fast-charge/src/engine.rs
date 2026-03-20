/// DC fast charge: CCS, CHAdeMO, power stage, cooling
/// Phase 854

#[derive(Debug, Clone)]
pub struct DcFastCharge {
    pub ccs_ok: bool,
    pub chademo_ok: bool,
    pub power_ok: bool,
    pub cooling_ok: bool,
    pub comm_ok: bool,
}

impl Default for DcFastCharge {
    fn default() -> Self {
        Self::new()
    }
}

impl DcFastCharge {
    pub fn new() -> Self {
        Self {
            ccs_ok: true,
            chademo_ok: true,
            power_ok: true,
            cooling_ok: true,
            comm_ok: true,
        }
    }

    pub fn protocol_ok(&self) -> bool {
        self.ccs_ok && self.chademo_ok && self.comm_ok
    }

    pub fn hardware_ok(&self) -> bool {
        self.power_ok && self.cooling_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protocol_ok() && self.hardware_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.power_ok || !self.cooling_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.power_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol() {
        let c = DcFastCharge::new();
        assert!(c.protocol_ok());
    }

    #[test]
    fn test_hardware() {
        let c = DcFastCharge::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DcFastCharge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DcFastCharge::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_power() {
        let mut c = DcFastCharge::new();
        c.power_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DcFastCharge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
