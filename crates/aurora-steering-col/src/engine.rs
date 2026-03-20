/// Steering column: tilt/telescoping, lock, universal joint
/// Phase 224

#[derive(Debug, Clone)]
pub struct SteeringColumn {
    pub tilt_deg: f64,
    pub telescope_mm: f64,
    pub locked: bool,
    pub u_joint_play_deg: f64,
    pub electric_adjust: bool,
    pub memory_position: u8,
}

impl Default for SteeringColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringColumn {
    pub fn new() -> Self {
        Self {
            tilt_deg: 0.0,
            telescope_mm: 0.0,
            locked: false,
            u_joint_play_deg: 0.2,
            electric_adjust: true,
            memory_position: 1,
        }
    }

    pub fn u_joint_ok(&self) -> bool {
        self.u_joint_play_deg < 1.5
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn in_range(&self) -> bool {
        self.tilt_deg.abs() < 15.0 && self.telescope_mm.abs() < 50.0
    }

    pub fn needs_service(&self) -> bool {
        !self.u_joint_ok()
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.u_joint_ok() {
            score -= 40.0;
        }
        if !self.in_range() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u_joint_ok() {
        let s = SteeringColumn::new();
        assert!(s.u_joint_ok());
    }

    #[test]
    fn test_not_locked() {
        let s = SteeringColumn::new();
        assert!(!s.is_locked());
    }

    #[test]
    fn test_in_range() {
        let s = SteeringColumn::new();
        assert!(s.in_range());
    }

    #[test]
    fn test_no_service() {
        let s = SteeringColumn::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_worn_u_joint() {
        let mut s = SteeringColumn::new();
        s.u_joint_play_deg = 3.0;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SteeringColumn::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
