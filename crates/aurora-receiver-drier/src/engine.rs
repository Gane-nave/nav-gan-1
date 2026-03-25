/// Receiver drier: desiccant, filter, moisture
/// Phase 625

#[derive(Debug, Clone)]
pub struct ReceiverDrier {
    pub desiccant_ok: bool,
    pub filter_ok: bool,
    pub moisture_ok: bool,
    pub sight_glass_ok: bool,
    pub capacity_ok: bool,
}

impl Default for ReceiverDrier {
    fn default() -> Self {
        Self::new()
    }
}

impl ReceiverDrier {
    pub fn new() -> Self {
        Self {
            desiccant_ok: true,
            filter_ok: true,
            moisture_ok: true,
            sight_glass_ok: true,
            capacity_ok: true,
        }
    }

    pub fn drying_ok(&self) -> bool {
        self.desiccant_ok && self.moisture_ok
    }

    pub fn filtration_ok(&self) -> bool {
        self.filter_ok && self.capacity_ok
    }

    pub fn all_ok(&self) -> bool {
        self.drying_ok() && self.filtration_ok() && self.sight_glass_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.desiccant_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.desiccant_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drying() {
        let c = ReceiverDrier::new();
        assert!(c.drying_ok());
    }

    #[test]
    fn test_filtration() {
        let c = ReceiverDrier::new();
        assert!(c.filtration_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReceiverDrier::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ReceiverDrier::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_desiccant() {
        let mut c = ReceiverDrier::new();
        c.desiccant_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ReceiverDrier::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
