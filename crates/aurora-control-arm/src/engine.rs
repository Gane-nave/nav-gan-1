/// Control arm: ball joint, bushing, geometry
/// Phase 478

#[derive(Debug, Clone)]
pub struct ControlArm {
    pub ball_joint_play_mm: f64,
    pub max_play_mm: f64,
    pub bushing_ok: bool,
    pub bent: bool,
    pub corroded: bool,
}

impl Default for ControlArm {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlArm {
    pub fn new() -> Self {
        Self {
            ball_joint_play_mm: 0.5,
            max_play_mm: 3.0,
            bushing_ok: true,
            bent: false,
            corroded: false,
        }
    }

    pub fn play_pct(&self) -> f64 {
        (self.ball_joint_play_mm / self.max_play_mm) * 100.0
    }

    pub fn excessive_play(&self) -> bool {
        self.ball_joint_play_mm > self.max_play_mm * 0.8
    }

    pub fn all_ok(&self) -> bool {
        !self.excessive_play() && self.bushing_ok && !self.bent
    }

    pub fn needs_replacement(&self) -> bool {
        self.bent || self.ball_joint_play_mm > self.max_play_mm
    }

    pub fn health_score(&self) -> f64 {
        if self.bent { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let c = ControlArm::new();
        assert!(c.play_pct() < 25.0);
    }

    #[test]
    fn test_no_excessive() {
        let c = ControlArm::new();
        assert!(!c.excessive_play());
    }

    #[test]
    fn test_all_ok() {
        let c = ControlArm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ControlArm::new();
        assert!(!c.needs_replacement());
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
