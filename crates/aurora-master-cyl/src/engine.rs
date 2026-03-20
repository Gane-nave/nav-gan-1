/// Master cylinder: bore, piston, reservoir, seal
/// Phase 658

#[derive(Debug, Clone)]
pub struct MasterCylinder {
    pub bore_ok: bool,
    pub piston_ok: bool,
    pub reservoir_ok: bool,
    pub seal_ok: bool,
    pub pressure_ok: bool,
}

impl Default for MasterCylinder {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterCylinder {
    pub fn new() -> Self {
        Self {
            bore_ok: true,
            piston_ok: true,
            reservoir_ok: true,
            seal_ok: true,
            pressure_ok: true,
        }
    }

    pub fn hydraulic_ok(&self) -> bool {
        self.bore_ok && self.piston_ok && self.seal_ok
    }

    pub fn supply_ok(&self) -> bool {
        self.reservoir_ok && self.pressure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.hydraulic_ok() && self.supply_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.bore_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bore_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydraulic() {
        let c = MasterCylinder::new();
        assert!(c.hydraulic_ok());
    }

    #[test]
    fn test_supply() {
        let c = MasterCylinder::new();
        assert!(c.supply_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MasterCylinder::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = MasterCylinder::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_bore() {
        let mut c = MasterCylinder::new();
        c.bore_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = MasterCylinder::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
