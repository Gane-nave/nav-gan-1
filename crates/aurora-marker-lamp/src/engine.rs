/// Marker lamp: side marker, clearance light, reflector condition
/// Phase 422

#[derive(Debug, Clone)]
pub struct MarkerLamp {
    pub left_ok: bool,
    pub right_ok: bool,
    pub front_ok: bool,
    pub rear_ok: bool,
    pub reflector_ok: bool,
}

impl Default for MarkerLamp {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkerLamp {
    pub fn new() -> Self {
        Self {
            left_ok: true,
            right_ok: true,
            front_ok: true,
            rear_ok: true,
            reflector_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.left_ok && self.right_ok && self.front_ok && self.rear_ok
    }

    pub fn compliant(&self) -> bool {
        self.all_ok() && self.reflector_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.left_ok || !self.right_ok
    }

    pub fn working_count(&self) -> u8 {
        [self.left_ok, self.right_ok, self.front_ok, self.rear_ok]
            .iter()
            .filter(|&&x| x)
            .count() as u8
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let m = MarkerLamp::new();
        assert!(m.all_ok());
    }

    #[test]
    fn test_compliant() {
        let m = MarkerLamp::new();
        assert!(m.compliant());
    }

    #[test]
    fn test_no_replace() {
        let m = MarkerLamp::new();
        assert!(!m.needs_replacement());
    }

    #[test]
    fn test_count() {
        let m = MarkerLamp::new();
        assert_eq!(m.working_count(), 4);
    }

    #[test]
    fn test_failure() {
        let mut m = MarkerLamp::new();
        m.left_ok = false;
        assert!(m.needs_replacement());
    }

    #[test]
    fn test_health() {
        let m = MarkerLamp::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
