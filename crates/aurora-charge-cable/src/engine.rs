/// Charge cable: conductor, sheath, plug, ground
/// Phase 853

#[derive(Debug, Clone)]
pub struct ChargeCable {
    pub conductor_ok: bool,
    pub sheath_ok: bool,
    pub plug_ok: bool,
    pub ground_ok: bool,
    pub rating_ok: bool,
}

impl Default for ChargeCable {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargeCable {
    pub fn new() -> Self {
        Self {
            conductor_ok: true,
            sheath_ok: true,
            plug_ok: true,
            ground_ok: true,
            rating_ok: true,
        }
    }

    pub fn electrical_ok(&self) -> bool {
        self.conductor_ok && self.ground_ok && self.rating_ok
    }

    pub fn physical_ok(&self) -> bool {
        self.sheath_ok && self.plug_ok
    }

    pub fn all_ok(&self) -> bool {
        self.electrical_ok() && self.physical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.conductor_ok || !self.sheath_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.conductor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical() {
        let c = ChargeCable::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_physical() {
        let c = ChargeCable::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargeCable::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ChargeCable::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_conductor() {
        let mut c = ChargeCable::new();
        c.conductor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ChargeCable::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
