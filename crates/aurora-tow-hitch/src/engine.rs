/// Tow hitch: receiver, ball mount, wiring, capacity
/// Phase 557

#[derive(Debug, Clone)]
pub struct TowHitch {
    pub capacity_kg: f64,
    pub load_kg: f64,
    pub wiring_ok: bool,
    pub ball_ok: bool,
    pub receiver_ok: bool,
}

impl Default for TowHitch {
    fn default() -> Self {
        Self::new()
    }
}

impl TowHitch {
    pub fn new() -> Self {
        Self {
            capacity_kg: 2500.0,
            load_kg: 500.0,
            wiring_ok: true,
            ball_ok: true,
            receiver_ok: true,
        }
    }

    pub fn within_capacity(&self) -> bool {
        self.load_kg < self.capacity_kg
    }

    pub fn hardware_ok(&self) -> bool {
        self.ball_ok && self.receiver_ok
    }

    pub fn all_ok(&self) -> bool {
        self.within_capacity() && self.hardware_ok() && self.wiring_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.receiver_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.receiver_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity() {
        let c = TowHitch::new();
        assert!(c.within_capacity());
    }

    #[test]
    fn test_hardware() {
        let c = TowHitch::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TowHitch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TowHitch::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_receiver() {
        let mut c = TowHitch::new();
        c.receiver_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TowHitch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
