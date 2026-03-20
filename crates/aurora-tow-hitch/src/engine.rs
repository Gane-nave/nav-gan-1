/// Tow hitch: receiver, tongue weight, trailer connection
/// Phase 335

#[derive(Debug, Clone)]
pub struct TowHitch {
    pub connected: bool,
    pub tongue_weight_kg: f64,
    pub max_tongue_kg: f64,
    pub tow_weight_kg: f64,
    pub max_tow_kg: f64,
    pub electrical_ok: bool,
}

impl Default for TowHitch {
    fn default() -> Self {
        Self::new()
    }
}

impl TowHitch {
    pub fn new() -> Self {
        Self {
            connected: false,
            tongue_weight_kg: 0.0,
            max_tongue_kg: 200.0,
            tow_weight_kg: 0.0,
            max_tow_kg: 2000.0,
            electrical_ok: true,
        }
    }

    pub fn trailer_connected(&self) -> bool {
        self.connected
    }

    pub fn tongue_ok(&self) -> bool {
        self.tongue_weight_kg <= self.max_tongue_kg
    }

    pub fn tow_ok(&self) -> bool {
        self.tow_weight_kg <= self.max_tow_kg
    }

    pub fn overloaded(&self) -> bool {
        self.tow_weight_kg > self.max_tow_kg || self.tongue_weight_kg > self.max_tongue_kg
    }

    pub fn health_score(&self) -> f64 {
        if self.overloaded() {
            return 20.0;
        }
        if !self.electrical_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_connected() {
        let t = TowHitch::new();
        assert!(!t.trailer_connected());
    }

    #[test]
    fn test_tongue_ok() {
        let t = TowHitch::new();
        assert!(t.tongue_ok());
    }

    #[test]
    fn test_tow_ok() {
        let t = TowHitch::new();
        assert!(t.tow_ok());
    }

    #[test]
    fn test_not_overloaded() {
        let t = TowHitch::new();
        assert!(!t.overloaded());
    }

    #[test]
    fn test_overloaded() {
        let mut t = TowHitch::new();
        t.tow_weight_kg = 3000.0;
        assert!(t.overloaded());
    }

    #[test]
    fn test_health() {
        let t = TowHitch::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
