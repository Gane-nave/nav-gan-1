/// Immobilizer: transponder authentication, engine lock, anti-theft
/// Phase 254

#[derive(Debug, Clone)]
pub struct Immobilizer {
    pub authenticated: bool,
    pub engine_locked: bool,
    pub transponder_detected: bool,
    pub tamper_detected: bool,
    pub system_armed: bool,
}

impl Default for Immobilizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Immobilizer {
    pub fn new() -> Self {
        Self {
            authenticated: true,
            engine_locked: false,
            transponder_detected: true,
            tamper_detected: false,
            system_armed: true,
        }
    }

    pub fn can_start(&self) -> bool {
        self.authenticated && self.transponder_detected && !self.engine_locked
    }

    pub fn is_secure(&self) -> bool {
        self.system_armed && !self.tamper_detected
    }

    pub fn theft_attempt(&self) -> bool {
        self.tamper_detected && self.system_armed
    }

    pub fn should_lock(&self) -> bool {
        !self.transponder_detected && self.system_armed
    }

    pub fn health_score(&self) -> f64 {
        if self.tamper_detected {
            return 20.0;
        }
        if !self.system_armed {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_start() {
        let i = Immobilizer::new();
        assert!(i.can_start());
    }

    #[test]
    fn test_secure() {
        let i = Immobilizer::new();
        assert!(i.is_secure());
    }

    #[test]
    fn test_no_theft() {
        let i = Immobilizer::new();
        assert!(!i.theft_attempt());
    }

    #[test]
    fn test_no_lock() {
        let i = Immobilizer::new();
        assert!(!i.should_lock());
    }

    #[test]
    fn test_tamper() {
        let mut i = Immobilizer::new();
        i.tamper_detected = true;
        assert!(i.theft_attempt());
    }

    #[test]
    fn test_health() {
        let i = Immobilizer::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
