/// Control arm monitoring: bushing wear, ball joint, geometry
/// Phase 201

#[derive(Debug, Clone)]
pub struct ControlArm {
    pub position: String,
    pub bushing_wear_pct: f64,
    pub ball_joint_play_mm: f64,
    pub bent: bool,
    pub mileage_km: f64,
}

impl Default for ControlArm {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlArm {
    pub fn new() -> Self {
        Self {
            position: "front_lower_left".into(),
            bushing_wear_pct: 20.0,
            ball_joint_play_mm: 0.3,
            bent: false,
            mileage_km: 70000.0,
        }
    }

    pub fn bushing_ok(&self) -> bool {
        self.bushing_wear_pct < 60.0
    }

    pub fn ball_joint_ok(&self) -> bool {
        self.ball_joint_play_mm < 1.5
    }

    pub fn needs_replacement(&self) -> bool {
        self.bent || self.bushing_wear_pct > 80.0 || self.ball_joint_play_mm > 2.0
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.bushing_wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.bushing_ok() {
            score -= 25.0;
        }
        if !self.ball_joint_ok() {
            score -= 25.0;
        }
        if self.bent {
            score -= 50.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let c = ControlArm::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bushing_ok() {
        let c = ControlArm::new();
        assert!(c.bushing_ok());
    }

    #[test]
    fn test_ball_joint_ok() {
        let c = ControlArm::new();
        assert!(c.ball_joint_ok());
    }

    #[test]
    fn test_remaining_life() {
        let c = ControlArm::new();
        assert!((c.remaining_life_pct() - 80.0).abs() < 0.1);
    }

    #[test]
    fn test_bent() {
        let mut c = ControlArm::new();
        c.bent = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ControlArm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
