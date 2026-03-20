/// Inverter: IGBT, DC link, gate driver, cooling
/// Phase 717

#[derive(Debug, Clone)]
pub struct Inverter {
    pub igbt_ok: bool,
    pub dc_link_ok: bool,
    pub gate_ok: bool,
    pub cooling_ok: bool,
    pub efficiency_ok: bool,
}

impl Default for Inverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Inverter {
    pub fn new() -> Self {
        Self {
            igbt_ok: true,
            dc_link_ok: true,
            gate_ok: true,
            cooling_ok: true,
            efficiency_ok: true,
        }
    }

    pub fn power_stage_ok(&self) -> bool {
        self.igbt_ok && self.dc_link_ok && self.gate_ok
    }

    pub fn thermal_ok(&self) -> bool {
        self.cooling_ok && self.efficiency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.power_stage_ok() && self.thermal_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.igbt_ok || !self.cooling_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.igbt_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_stage() {
        let c = Inverter::new();
        assert!(c.power_stage_ok());
    }

    #[test]
    fn test_thermal() {
        let c = Inverter::new();
        assert!(c.thermal_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Inverter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Inverter::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_igbt() {
        let mut c = Inverter::new();
        c.igbt_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Inverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
