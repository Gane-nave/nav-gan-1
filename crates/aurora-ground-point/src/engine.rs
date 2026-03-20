/// Ground point: chassis ground, resistance, corrosion
/// Phase 523

#[derive(Debug, Clone)]
pub struct GroundPoint {
    pub resistance_mohm: f64,
    pub max_resistance_mohm: f64,
    pub bolt_tight: bool,
    pub corrosion_free: bool,
    pub paint_clear: bool,
}

impl Default for GroundPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl GroundPoint {
    pub fn new() -> Self {
        Self {
            resistance_mohm: 5.0,
            max_resistance_mohm: 50.0,
            bolt_tight: true,
            corrosion_free: true,
            paint_clear: true,
        }
    }

    pub fn resistance_ok(&self) -> bool {
        self.resistance_mohm < self.max_resistance_mohm
    }

    pub fn connection_ok(&self) -> bool {
        self.bolt_tight && self.corrosion_free && self.paint_clear
    }

    pub fn all_ok(&self) -> bool {
        self.resistance_ok() && self.connection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.corrosion_free || !self.bolt_tight
    }

    pub fn health_score(&self) -> f64 {
        if !self.bolt_tight {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistance() {
        let c = GroundPoint::new();
        assert!(c.resistance_ok());
    }

    #[test]
    fn test_connection() {
        let c = GroundPoint::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GroundPoint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GroundPoint::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_corroded() {
        let mut c = GroundPoint::new();
        c.corrosion_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GroundPoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
