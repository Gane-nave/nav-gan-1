/// Brake cooling: duct airflow, rotor temperature, fade prevention
/// Phase 360

#[derive(Debug, Clone)]
pub struct BrakeCool {
    pub duct_open: bool,
    pub rotor_temp_c: f64,
    pub max_temp_c: f64,
    pub airflow_ok: bool,
    pub fade_risk: bool,
}

impl Default for BrakeCool {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeCool {
    pub fn new() -> Self {
        Self {
            duct_open: true,
            rotor_temp_c: 200.0,
            max_temp_c: 700.0,
            airflow_ok: true,
            fade_risk: false,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.rotor_temp_c < self.max_temp_c * 0.8
    }

    pub fn overheating(&self) -> bool {
        self.rotor_temp_c > self.max_temp_c * 0.9
    }

    pub fn cooling_effective(&self) -> bool {
        self.duct_open && self.airflow_ok
    }

    pub fn needs_attention(&self) -> bool {
        self.fade_risk || self.overheating()
    }

    pub fn health_score(&self) -> f64 {
        if self.overheating() {
            return 10.0;
        }
        if self.fade_risk {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_ok() {
        let b = BrakeCool::new();
        assert!(b.temp_ok());
    }

    #[test]
    fn test_not_overheating() {
        let b = BrakeCool::new();
        assert!(!b.overheating());
    }

    #[test]
    fn test_cooling() {
        let b = BrakeCool::new();
        assert!(b.cooling_effective());
    }

    #[test]
    fn test_no_attention() {
        let b = BrakeCool::new();
        assert!(!b.needs_attention());
    }

    #[test]
    fn test_fade() {
        let mut b = BrakeCool::new();
        b.fade_risk = true;
        assert!(b.needs_attention());
    }

    #[test]
    fn test_health() {
        let b = BrakeCool::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
