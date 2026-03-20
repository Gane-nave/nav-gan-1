/// Coolant: concentration, pH, level, freeze point
/// Phase 567

#[derive(Debug, Clone)]
pub struct Coolant {
    pub concentration_pct: f64,
    pub ph_level: f64,
    pub level_ok: bool,
    pub freeze_point_c: f64,
    pub color_ok: bool,
}

impl Default for Coolant {
    fn default() -> Self {
        Self::new()
    }
}

impl Coolant {
    pub fn new() -> Self {
        Self {
            concentration_pct: 50.0,
            ph_level: 8.5,
            level_ok: true,
            freeze_point_c: -37.0,
            color_ok: true,
        }
    }

    pub fn concentration_ok(&self) -> bool {
        self.concentration_pct > 40.0 && self.concentration_pct < 70.0
    }

    pub fn ph_ok(&self) -> bool {
        self.ph_level > 7.0 && self.ph_level < 11.0
    }

    pub fn all_ok(&self) -> bool {
        self.concentration_ok() && self.ph_ok() && self.level_ok && self.color_ok
    }

    pub fn needs_change(&self) -> bool {
        !self.level_ok || !self.color_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.level_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concentration() {
        let c = Coolant::new();
        assert!(c.concentration_ok());
    }

    #[test]
    fn test_ph() {
        let c = Coolant::new();
        assert!(c.ph_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Coolant::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = Coolant::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_level() {
        let mut c = Coolant::new();
        c.level_ok = false;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = Coolant::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
