/// Belt inspection: tension, crack, alignment, wear
/// Phase 829

#[derive(Debug, Clone)]
pub struct BeltInspect {
    pub tension_ok: bool,
    pub crack_free: bool,
    pub aligned: bool,
    pub wear_ok: bool,
    pub pulley_ok: bool,
}

impl Default for BeltInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl BeltInspect {
    pub fn new() -> Self {
        Self {
            tension_ok: true,
            crack_free: true,
            aligned: true,
            wear_ok: true,
            pulley_ok: true,
        }
    }

    pub fn drive_ok(&self) -> bool {
        self.tension_ok && self.aligned && self.pulley_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.crack_free && self.wear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.drive_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.crack_free || !self.tension_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.crack_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive() {
        let c = BeltInspect::new();
        assert!(c.drive_ok());
    }

    #[test]
    fn test_condition() {
        let c = BeltInspect::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BeltInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BeltInspect::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = BeltInspect::new();
        c.crack_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BeltInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
