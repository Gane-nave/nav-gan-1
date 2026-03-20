/// Climate ECU: temperature, fan, mode, recirculation
/// Phase 712

#[derive(Debug, Clone)]
pub struct ClimateEcu {
    pub temp_ctrl_ok: bool,
    pub fan_ctrl_ok: bool,
    pub mode_ok: bool,
    pub recirc_ok: bool,
    pub comm_ok: bool,
}

impl Default for ClimateEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl ClimateEcu {
    pub fn new() -> Self {
        Self {
            temp_ctrl_ok: true,
            fan_ctrl_ok: true,
            mode_ok: true,
            recirc_ok: true,
            comm_ok: true,
        }
    }

    pub fn hvac_ok(&self) -> bool {
        self.temp_ctrl_ok && self.fan_ctrl_ok && self.mode_ok
    }

    pub fn air_ok(&self) -> bool {
        self.recirc_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.hvac_ok() && self.air_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.temp_ctrl_ok || !self.fan_ctrl_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.temp_ctrl_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hvac() {
        let c = ClimateEcu::new();
        assert!(c.hvac_ok());
    }

    #[test]
    fn test_air() {
        let c = ClimateEcu::new();
        assert!(c.air_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClimateEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ClimateEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_temp() {
        let mut c = ClimateEcu::new();
        c.temp_ctrl_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ClimateEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
