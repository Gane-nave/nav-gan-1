/// seat ctrl: position, heat, cool, massage, memory
/// Phase 1308

#[derive(Debug, Clone)]
pub struct SeatCtrl2 {
    pub position_ok: bool,
    pub heat_ok: bool,
    pub cool_ok: bool,
    pub massage_ok: bool,
    pub memory_ok: bool,
}

impl Default for SeatCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatCtrl2 {
    pub fn new() -> Self {
        Self {
            position_ok: true,
            heat_ok: true,
            cool_ok: true,
            massage_ok: true,
            memory_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.position_ok && self.heat_ok && self.cool_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.massage_ok && self.memory_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.position_ok || !self.heat_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.position_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SeatCtrl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SeatCtrl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatCtrl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SeatCtrl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SeatCtrl2::new();
        c.position_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SeatCtrl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
