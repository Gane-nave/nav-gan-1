/// Wheel lock: key pattern, socket, anti-theft, grade
/// Phase 810

#[derive(Debug, Clone)]
pub struct WheelLock {
    pub key_ok: bool,
    pub socket_ok: bool,
    pub anti_theft_ok: bool,
    pub grade_ok: bool,
    pub torque_ok: bool,
}

impl Default for WheelLock {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelLock {
    pub fn new() -> Self {
        Self {
            key_ok: true,
            socket_ok: true,
            anti_theft_ok: true,
            grade_ok: true,
            torque_ok: true,
        }
    }

    pub fn security_ok(&self) -> bool {
        self.key_ok && self.anti_theft_ok
    }

    pub fn fastening_ok(&self) -> bool {
        self.socket_ok && self.grade_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.security_ok() && self.fastening_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.key_ok || !self.torque_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.key_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        let c = WheelLock::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_fastening() {
        let c = WheelLock::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WheelLock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WheelLock::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_key() {
        let mut c = WheelLock::new();
        c.key_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WheelLock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
