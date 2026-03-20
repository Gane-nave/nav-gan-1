/// Transmission fluid: level, color, temp, pressure
/// Phase 565

#[derive(Debug, Clone)]
pub struct TransFluid {
    pub level_pct: f64,
    pub color_ok: bool,
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub pressure_ok: bool,
}

impl Default for TransFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl TransFluid {
    pub fn new() -> Self {
        Self {
            level_pct: 85.0,
            color_ok: true,
            temp_c: 80.0,
            max_temp_c: 120.0,
            pressure_ok: true,
        }
    }

    pub fn level_ok(&self) -> bool {
        self.level_pct > 30.0
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.level_ok() && self.temp_ok() && self.color_ok && self.pressure_ok
    }

    pub fn needs_change(&self) -> bool {
        !self.color_ok || self.level_pct < 20.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.color_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        let c = TransFluid::new();
        assert!(c.level_ok());
    }

    #[test]
    fn test_temp() {
        let c = TransFluid::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_change() {
        let c = TransFluid::new();
        assert!(!c.needs_change());
    }

    #[test]
    fn test_color() {
        let mut c = TransFluid::new();
        c.color_ok = false;
        assert!(c.needs_change());
    }

    #[test]
    fn test_health() {
        let c = TransFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
