/// Tow hitch: receiver, ball mount, wiring, controller
/// Phase 759

#[derive(Debug, Clone)]
pub struct TowHitch {
    pub receiver_ok: bool,
    pub ball_ok: bool,
    pub wiring_ok: bool,
    pub controller_ok: bool,
    pub rating_ok: bool,
}

impl Default for TowHitch {
    fn default() -> Self {
        Self::new()
    }
}

impl TowHitch {
    pub fn new() -> Self {
        Self {
            receiver_ok: true,
            ball_ok: true,
            wiring_ok: true,
            controller_ok: true,
            rating_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.receiver_ok && self.ball_ok && self.rating_ok
    }

    pub fn electrical_ok(&self) -> bool {
        self.wiring_ok && self.controller_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.electrical_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.receiver_ok || !self.wiring_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.receiver_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = TowHitch::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_electrical() {
        let c = TowHitch::new();
        assert!(c.electrical_ok());
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
