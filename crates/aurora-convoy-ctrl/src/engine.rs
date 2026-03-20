/// Convoy control: form, join, coordinate, separate, emergency
/// Phase 1121

#[derive(Debug, Clone)]
pub struct ConvoyCtrl {
    pub form_ok: bool,
    pub join_ok: bool,
    pub coordinate_ok: bool,
    pub separate_ok: bool,
    pub emergency_ok: bool,
}

impl Default for ConvoyCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl ConvoyCtrl {
    pub fn new() -> Self {
        Self {
            form_ok: true,
            join_ok: true,
            coordinate_ok: true,
            separate_ok: true,
            emergency_ok: true,
        }
    }

    pub fn formation_ok(&self) -> bool {
        self.form_ok && self.join_ok && self.coordinate_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.separate_ok && self.emergency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.formation_ok() && self.safety_ok()
    }

    pub fn needs_reform(&self) -> bool {
        !self.form_ok || !self.coordinate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.form_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation() {
        let c = ConvoyCtrl::new();
        assert!(c.formation_ok());
    }

    #[test]
    fn test_safety() {
        let c = ConvoyCtrl::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConvoyCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reform() {
        let c = ConvoyCtrl::new();
        assert!(!c.needs_reform());
    }

    #[test]
    fn test_form() {
        let mut c = ConvoyCtrl::new();
        c.form_ok = false;
        assert!(c.needs_reform());
    }

    #[test]
    fn test_health() {
        let c = ConvoyCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
