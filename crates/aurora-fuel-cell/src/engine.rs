/// fuel cell: start, generate, cool, purge, shutdown
/// Phase 1145

#[derive(Debug, Clone)]
pub struct FuelCell {
    pub start_ok: bool,
    pub generate_ok: bool,
    pub cool_ok: bool,
    pub purge_ok: bool,
    pub shutdown_ok: bool,
}

impl Default for FuelCell {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelCell {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            generate_ok: true,
            cool_ok: true,
            purge_ok: true,
            shutdown_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.generate_ok && self.cool_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.purge_ok && self.shutdown_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.generate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FuelCell::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuelCell::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelCell::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuelCell::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuelCell::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuelCell::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
