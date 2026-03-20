/// Chassis flex: torsional rigidity, flex measurement, reinforcement
/// Phase 334

#[derive(Debug, Clone)]
pub struct ChassisFlex {
    pub torsional_rigidity_nm_deg: f64,
    pub flex_measurement_mm: f64,
    pub max_flex_mm: f64,
    pub reinforced: bool,
    pub fatigue_cycles: u64,
}

impl Default for ChassisFlex {
    fn default() -> Self {
        Self::new()
    }
}

impl ChassisFlex {
    pub fn new() -> Self {
        Self {
            torsional_rigidity_nm_deg: 25000.0,
            flex_measurement_mm: 1.0,
            max_flex_mm: 5.0,
            reinforced: false,
            fatigue_cycles: 100000,
        }
    }

    pub fn flex_ok(&self) -> bool {
        self.flex_measurement_mm < self.max_flex_mm
    }

    pub fn rigid_enough(&self) -> bool {
        self.torsional_rigidity_nm_deg > 20000.0
    }

    pub fn excess_flex(&self) -> bool {
        self.flex_measurement_mm > self.max_flex_mm * 0.8
    }

    pub fn fatigue_ok(&self) -> bool {
        self.fatigue_cycles < 500000
    }

    pub fn health_score(&self) -> f64 {
        if !self.flex_ok() {
            return 0.0;
        }
        if self.excess_flex() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_ok() {
        let c = ChassisFlex::new();
        assert!(c.flex_ok());
    }

    #[test]
    fn test_rigid() {
        let c = ChassisFlex::new();
        assert!(c.rigid_enough());
    }

    #[test]
    fn test_no_excess() {
        let c = ChassisFlex::new();
        assert!(!c.excess_flex());
    }

    #[test]
    fn test_fatigue() {
        let c = ChassisFlex::new();
        assert!(c.fatigue_ok());
    }

    #[test]
    fn test_too_flex() {
        let mut c = ChassisFlex::new();
        c.flex_measurement_mm = 6.0;
        assert!(!c.flex_ok());
    }

    #[test]
    fn test_health() {
        let c = ChassisFlex::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
