/// Lug nut: torque, thread condition, stud integrity
/// Phase 548

#[derive(Debug, Clone)]
pub struct LugNut {
    pub torque_nm: f64,
    pub target_torque_nm: f64,
    pub thread_ok: bool,
    pub stud_ok: bool,
    pub seated: bool,
}

impl Default for LugNut {
    fn default() -> Self {
        Self::new()
    }
}

impl LugNut {
    pub fn new() -> Self {
        Self {
            torque_nm: 110.0,
            target_torque_nm: 110.0,
            thread_ok: true,
            stud_ok: true,
            seated: true,
        }
    }

    pub fn torque_ok(&self) -> bool {
        (self.torque_nm - self.target_torque_nm).abs() < 15.0
    }

    pub fn thread_good(&self) -> bool {
        self.thread_ok && self.stud_ok
    }

    pub fn all_ok(&self) -> bool {
        self.torque_ok() && self.thread_good() && self.seated
    }

    pub fn needs_service(&self) -> bool {
        !self.thread_ok || !self.stud_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stud_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torque() {
        let c = LugNut::new();
        assert!(c.torque_ok());
    }

    #[test]
    fn test_thread() {
        let c = LugNut::new();
        assert!(c.thread_good());
    }

    #[test]
    fn test_all_ok() {
        let c = LugNut::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = LugNut::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_stud() {
        let mut c = LugNut::new();
        c.stud_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = LugNut::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
