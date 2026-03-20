/// Ball joint: wear, boot, play, grease
/// Phase 639

#[derive(Debug, Clone)]
pub struct BallJoint {
    pub wear_ok: bool,
    pub boot_ok: bool,
    pub play_mm: f64,
    pub max_play_mm: f64,
    pub greased: bool,
}

impl Default for BallJoint {
    fn default() -> Self {
        Self::new()
    }
}

impl BallJoint {
    pub fn new() -> Self {
        Self {
            wear_ok: true,
            boot_ok: true,
            play_mm: 0.5,
            max_play_mm: 2.0,
            greased: true,
        }
    }

    pub fn play_ok(&self) -> bool {
        self.play_mm < self.max_play_mm
    }

    pub fn condition_ok(&self) -> bool {
        self.wear_ok && self.boot_ok && self.greased
    }

    pub fn all_ok(&self) -> bool {
        self.play_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.wear_ok || !self.boot_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wear_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let c = BallJoint::new();
        assert!(c.play_ok());
    }

    #[test]
    fn test_condition() {
        let c = BallJoint::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BallJoint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BallJoint::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_wear() {
        let mut c = BallJoint::new();
        c.wear_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BallJoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
