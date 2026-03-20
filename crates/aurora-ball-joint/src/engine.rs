/// Ball joint monitoring: play measurement, boot condition, load rating
/// Phase 203

#[derive(Debug, Clone)]
pub struct BallJoint {
    pub position: String,
    pub axial_play_mm: f64,
    pub radial_play_mm: f64,
    pub boot_intact: bool,
    pub load_rating_kn: f64,
    pub current_load_kn: f64,
}

impl Default for BallJoint {
    fn default() -> Self {
        Self::new()
    }
}

impl BallJoint {
    pub fn new() -> Self {
        Self {
            position: "lower_left".into(),
            axial_play_mm: 0.1,
            radial_play_mm: 0.05,
            boot_intact: true,
            load_rating_kn: 25.0,
            current_load_kn: 8.0,
        }
    }

    pub fn axial_ok(&self) -> bool {
        self.axial_play_mm < 0.5
    }

    pub fn radial_ok(&self) -> bool {
        self.radial_play_mm < 0.25
    }

    pub fn overloaded(&self) -> bool {
        self.current_load_kn > self.load_rating_kn * 0.9
    }

    pub fn load_margin_pct(&self) -> f64 {
        if self.load_rating_kn <= 0.0 {
            return 0.0;
        }
        ((1.0 - self.current_load_kn / self.load_rating_kn) * 100.0).max(0.0)
    }

    pub fn needs_replacement(&self) -> bool {
        !self.axial_ok() || !self.radial_ok() || !self.boot_intact
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.axial_ok() {
            score -= 30.0;
        }
        if !self.radial_ok() {
            score -= 25.0;
        }
        if !self.boot_intact {
            score -= 25.0;
        }
        if self.overloaded() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let b = BallJoint::new();
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_axial_ok() {
        let b = BallJoint::new();
        assert!(b.axial_ok());
    }

    #[test]
    fn test_radial_ok() {
        let b = BallJoint::new();
        assert!(b.radial_ok());
    }

    #[test]
    fn test_not_overloaded() {
        let b = BallJoint::new();
        assert!(!b.overloaded());
    }

    #[test]
    fn test_load_margin() {
        let b = BallJoint::new();
        assert!(b.load_margin_pct() > 60.0);
    }

    #[test]
    fn test_torn_boot() {
        let mut b = BallJoint::new();
        b.boot_intact = false;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BallJoint::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
