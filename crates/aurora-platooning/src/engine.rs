/// Vehicle platooning: form, join, leave, coordinate, emergency
/// Phase 1102

#[derive(Debug, Clone)]
pub struct Platooning {
    pub form_ok: bool,
    pub join_ok: bool,
    pub leave_ok: bool,
    pub coordinate_ok: bool,
    pub emergency_ok: bool,
}

impl Default for Platooning {
    fn default() -> Self {
        Self::new()
    }
}

impl Platooning {
    pub fn new() -> Self {
        Self {
            form_ok: true,
            join_ok: true,
            leave_ok: true,
            coordinate_ok: true,
            emergency_ok: true,
        }
    }

    pub fn formation_ok(&self) -> bool {
        self.form_ok && self.join_ok && self.leave_ok
    }

    pub fn control_ok(&self) -> bool {
        self.coordinate_ok && self.emergency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.formation_ok() && self.control_ok()
    }

    pub fn needs_reform(&self) -> bool {
        !self.form_ok || !self.coordinate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.form_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation() {
        let c = Platooning::new();
        assert!(c.formation_ok());
    }

    #[test]
    fn test_control() {
        let c = Platooning::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Platooning::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reform() {
        let c = Platooning::new();
        assert!(!c.needs_reform());
    }

    #[test]
    fn test_form() {
        let mut c = Platooning::new();
        c.form_ok = false;
        assert!(c.needs_reform());
    }

    #[test]
    fn test_health() {
        let c = Platooning::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
