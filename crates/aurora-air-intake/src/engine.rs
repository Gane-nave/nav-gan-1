/// Air intake: duct, resonator, snorkel, filter box
/// Phase 612

#[derive(Debug, Clone)]
pub struct AirIntake {
    pub duct_ok: bool,
    pub resonator_ok: bool,
    pub snorkel_ok: bool,
    pub filter_box_ok: bool,
    pub sealed: bool,
}

impl Default for AirIntake {
    fn default() -> Self {
        Self::new()
    }
}

impl AirIntake {
    pub fn new() -> Self {
        Self {
            duct_ok: true,
            resonator_ok: true,
            snorkel_ok: true,
            filter_box_ok: true,
            sealed: true,
        }
    }

    pub fn pathway_ok(&self) -> bool {
        self.duct_ok && self.snorkel_ok
    }

    pub fn housing_ok(&self) -> bool {
        self.filter_box_ok && self.resonator_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pathway_ok() && self.housing_ok() && self.sealed
    }

    pub fn needs_service(&self) -> bool {
        !self.duct_ok || !self.sealed
    }

    pub fn health_score(&self) -> f64 {
        if !self.sealed { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathway() {
        let c = AirIntake::new();
        assert!(c.pathway_ok());
    }

    #[test]
    fn test_housing() {
        let c = AirIntake::new();
        assert!(c.housing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AirIntake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AirIntake::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_seal() {
        let mut c = AirIntake::new();
        c.sealed = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AirIntake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
