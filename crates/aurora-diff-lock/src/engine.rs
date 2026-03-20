/// Differential lock: center/rear/front diff control, torque split
/// Phase 162

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiffPosition {
    Center,
    Front,
    Rear,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LockState {
    Open,
    Partial(u8),
    Locked,
}

impl LockState {
    pub fn lock_pct(&self) -> f64 {
        match self {
            LockState::Open => 0.0,
            LockState::Partial(p) => *p as f64,
            LockState::Locked => 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Differential {
    pub position: DiffPosition,
    pub state: LockState,
    pub torque_split_pct: f64,
}

impl Differential {
    pub fn new(position: DiffPosition) -> Self {
        Self {
            position,
            state: LockState::Open,
            torque_split_pct: 50.0,
        }
    }

    pub fn is_locked(&self) -> bool {
        matches!(self.state, LockState::Locked)
    }

    pub fn effective_traction(&self) -> f64 {
        50.0 + self.state.lock_pct() * 0.5
    }
}

#[derive(Debug, Clone)]
pub struct DiffLockSystem {
    pub diffs: Vec<Differential>,
    pub auto_mode: bool,
}

impl Default for DiffLockSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffLockSystem {
    pub fn new() -> Self {
        Self {
            diffs: vec![
                Differential::new(DiffPosition::Center),
                Differential::new(DiffPosition::Front),
                Differential::new(DiffPosition::Rear),
            ],
            auto_mode: true,
        }
    }

    pub fn any_locked(&self) -> bool {
        self.diffs.iter().any(|d| d.is_locked())
    }

    pub fn total_traction_score(&self) -> f64 {
        if self.diffs.is_empty() {
            return 0.0;
        }
        let total: f64 = self.diffs.iter().map(|d| d.effective_traction()).sum();
        total / self.diffs.len() as f64
    }

    pub fn all_open(&self) -> bool {
        self.diffs
            .iter()
            .all(|d| matches!(d.state, LockState::Open))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_pct() {
        assert!((LockState::Locked.lock_pct() - 100.0).abs() < 0.1);
        assert!((LockState::Open.lock_pct() - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_partial() {
        assert!((LockState::Partial(50).lock_pct() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_is_locked() {
        let mut d = Differential::new(DiffPosition::Center);
        d.state = LockState::Locked;
        assert!(d.is_locked());
    }

    #[test]
    fn test_effective_traction() {
        let mut d = Differential::new(DiffPosition::Rear);
        d.state = LockState::Locked;
        assert!(d.effective_traction() > 90.0);
    }

    #[test]
    fn test_system_all_open() {
        let s = DiffLockSystem::new();
        assert!(s.all_open());
    }

    #[test]
    fn test_system_none_locked() {
        let s = DiffLockSystem::new();
        assert!(!s.any_locked());
    }

    #[test]
    fn test_traction_score() {
        let s = DiffLockSystem::new();
        assert!(s.total_traction_score() > 40.0);
    }
}
