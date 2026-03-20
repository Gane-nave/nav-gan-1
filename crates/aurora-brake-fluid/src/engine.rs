/// Brake fluid: moisture, boiling point, level
/// Phase 566

#[derive(Debug, Clone)]
pub struct BrakeFluid {
    pub moisture_pct: f64,
    pub max_moisture_pct: f64,
    pub boiling_c: f64,
    pub level_ok: bool,
    pub color_ok: bool,
}

impl Default for BrakeFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeFluid {
    pub fn new() -> Self {
        Self {
            moisture_pct: 1.0,
            max_moisture_pct: 3.0,
            boiling_c: 230.0,
            level_ok: true,
            color_ok: true,
        }
    }

    pub fn moisture_ok(&self) -> bool {
        self.moisture_pct < self.max_moisture_pct
    }

    pub fn boiling_ok(&self) -> bool {
        self.boiling_c > 200.0
    }

    pub fn all_ok(&self) -> bool {
        self.moisture_ok() && self.boiling_ok() && self.level_ok && self.color_ok
    }

    pub fn needs_change(&self) -> bool {
        self.moisture_pct > self.max_moisture_pct || !self.level_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.moisture_pct > self.max_moisture_pct { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moisture() {
        let c = BrakeFluid::new();
        assert!(c.moisture_ok());
    }

    #[test]
    fn test_boiling() {
        let c = BrakeFluid::new();
        assert!(c.boiling_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = BrakeFluid::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_high_moisture() {
        let mut c = BrakeFluid::new();
        c.moisture_pct = 4.0;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = BrakeFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
