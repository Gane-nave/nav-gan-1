/// Aerodynamic coefficient: Cd, Cl, frontal area, Reynolds number
/// Phase 348

#[derive(Debug, Clone)]
pub struct AeroCoeff {
    pub drag_coeff: f64,
    pub lift_coeff: f64,
    pub frontal_area_m2: f64,
    pub speed_kmh: f64,
}

impl Default for AeroCoeff {
    fn default() -> Self {
        Self::new()
    }
}

impl AeroCoeff {
    pub fn new() -> Self {
        Self {
            drag_coeff: 0.28,
            lift_coeff: 0.05,
            frontal_area_m2: 2.2,
            speed_kmh: 100.0,
        }
    }

    pub fn drag_force_n(&self) -> f64 {
        let v = self.speed_kmh / 3.6;
        0.5 * 1.225 * self.drag_coeff * self.frontal_area_m2 * v * v
    }

    pub fn efficient(&self) -> bool {
        self.drag_coeff < 0.30
    }

    pub fn lift_ok(&self) -> bool {
        self.lift_coeff < 0.1
    }

    pub fn high_speed(&self) -> bool {
        self.speed_kmh > 120.0
    }

    pub fn health_score(&self) -> f64 {
        if self.drag_coeff > 0.40 {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_force() {
        let a = AeroCoeff::new();
        assert!(a.drag_force_n() > 200.0);
    }

    #[test]
    fn test_efficient() {
        let a = AeroCoeff::new();
        assert!(a.efficient());
    }

    #[test]
    fn test_lift() {
        let a = AeroCoeff::new();
        assert!(a.lift_ok());
    }

    #[test]
    fn test_not_high_speed() {
        let a = AeroCoeff::new();
        assert!(!a.high_speed());
    }

    #[test]
    fn test_high_drag() {
        let mut a = AeroCoeff::new();
        a.drag_coeff = 0.45;
        assert!(!a.efficient());
    }

    #[test]
    fn test_health() {
        let a = AeroCoeff::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
