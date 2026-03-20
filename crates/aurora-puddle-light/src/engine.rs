/// Puddle light: under-mirror, under-door, welcome projection
/// Phase 426

#[derive(Debug, Clone)]
pub struct PuddleLight {
    pub left_ok: bool,
    pub right_ok: bool,
    pub projector_ok: bool,
    pub auto_on: bool,
    pub brightness_ok: bool,
}

impl Default for PuddleLight {
    fn default() -> Self {
        Self::new()
    }
}

impl PuddleLight {
    pub fn new() -> Self {
        Self {
            left_ok: true,
            right_ok: true,
            projector_ok: true,
            auto_on: true,
            brightness_ok: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.left_ok && self.right_ok && self.brightness_ok
    }

    pub fn functional(&self) -> bool {
        self.left_ok || self.right_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.left_ok && !self.right_ok
    }

    pub fn welcome_ok(&self) -> bool {
        self.projector_ok && self.auto_on
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_replacement() {
            return 0.0;
        }
        if !self.all_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let p = PuddleLight::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_functional() {
        let p = PuddleLight::new();
        assert!(p.functional());
    }

    #[test]
    fn test_no_replace() {
        let p = PuddleLight::new();
        assert!(!p.needs_replacement());
    }

    #[test]
    fn test_welcome() {
        let p = PuddleLight::new();
        assert!(p.welcome_ok());
    }

    #[test]
    fn test_both_out() {
        let mut p = PuddleLight::new();
        p.left_ok = false;
        p.right_ok = false;
        assert!(p.needs_replacement());
    }

    #[test]
    fn test_health() {
        let p = PuddleLight::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
