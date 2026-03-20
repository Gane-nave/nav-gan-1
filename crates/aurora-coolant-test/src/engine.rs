/// Coolant test: freeze point, pH, conductivity, color
/// Phase 824

#[derive(Debug, Clone)]
pub struct CoolantTest {
    pub freeze_ok: bool,
    pub ph_ok: bool,
    pub conductivity_ok: bool,
    pub color_ok: bool,
    pub level_ok: bool,
}

impl Default for CoolantTest {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantTest {
    pub fn new() -> Self {
        Self {
            freeze_ok: true,
            ph_ok: true,
            conductivity_ok: true,
            color_ok: true,
            level_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.freeze_ok && self.ph_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.conductivity_ok && self.color_ok && self.level_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.condition_ok()
    }

    pub fn needs_flush(&self) -> bool {
        !self.ph_ok || !self.conductivity_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ph_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = CoolantTest::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_condition() {
        let c = CoolantTest::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CoolantTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_flush() {
        let c = CoolantTest::new();
        assert!(!c.needs_flush());
    }

    #[test]
    fn test_ph() {
        let mut c = CoolantTest::new();
        c.ph_ok = false;
        assert!(c.needs_flush());
    }

    #[test]
    fn test_health() {
        let c = CoolantTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
