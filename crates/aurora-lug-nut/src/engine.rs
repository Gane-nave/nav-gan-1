/// Lug nut: torque, thread, seat, cover
/// Phase 806

#[derive(Debug, Clone)]
pub struct LugNut {
    pub torque_ok: bool,
    pub thread_ok: bool,
    pub seat_ok: bool,
    pub cover_ok: bool,
    pub grade_ok: bool,
}

impl Default for LugNut {
    fn default() -> Self {
        Self::new()
    }
}

impl LugNut {
    pub fn new() -> Self {
        Self {
            torque_ok: true,
            thread_ok: true,
            seat_ok: true,
            cover_ok: true,
            grade_ok: true,
        }
    }

    pub fn fastening_ok(&self) -> bool {
        self.torque_ok && self.thread_ok && self.seat_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.cover_ok && self.grade_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fastening_ok() && self.condition_ok()
    }

    pub fn needs_retorque(&self) -> bool {
        !self.torque_ok || !self.thread_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.torque_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastening() {
        let c = LugNut::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_condition() {
        let c = LugNut::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LugNut::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retorque() {
        let c = LugNut::new();
        assert!(!c.needs_retorque());
    }

    #[test]
    fn test_torque() {
        let mut c = LugNut::new();
        c.torque_ok = false;
        assert!(c.needs_retorque());
    }

    #[test]
    fn test_health() {
        let c = LugNut::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
