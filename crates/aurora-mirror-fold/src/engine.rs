/// Mirror fold control: auto-fold, parking fold, speed-based adjustment
/// Phase 177

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MirrorState {
    Extended,
    Folded,
    Folding,
    Extending,
}

#[derive(Debug, Clone)]
pub struct MirrorFoldSystem {
    pub left_state: MirrorState,
    pub right_state: MirrorState,
    pub auto_fold_enabled: bool,
    pub fold_on_lock: bool,
    pub speed_kmh: f64,
}

impl Default for MirrorFoldSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorFoldSystem {
    pub fn new() -> Self {
        Self {
            left_state: MirrorState::Extended,
            right_state: MirrorState::Extended,
            auto_fold_enabled: true,
            fold_on_lock: true,
            speed_kmh: 0.0,
        }
    }

    pub fn both_extended(&self) -> bool {
        matches!(self.left_state, MirrorState::Extended)
            && matches!(self.right_state, MirrorState::Extended)
    }

    pub fn both_folded(&self) -> bool {
        matches!(self.left_state, MirrorState::Folded)
            && matches!(self.right_state, MirrorState::Folded)
    }

    pub fn should_auto_fold(&self) -> bool {
        self.auto_fold_enabled && self.speed_kmh < 1.0 && self.both_extended()
    }

    pub fn safe_to_extend(&self) -> bool {
        self.speed_kmh < 5.0 || self.both_extended()
    }

    pub fn drag_reduction_when_folded(&self) -> f64 {
        if self.both_folded() {
            0.02
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended() {
        let s = MirrorFoldSystem::new();
        assert!(s.both_extended());
    }

    #[test]
    fn test_not_folded() {
        let s = MirrorFoldSystem::new();
        assert!(!s.both_folded());
    }

    #[test]
    fn test_auto_fold() {
        let s = MirrorFoldSystem::new();
        assert!(s.should_auto_fold());
    }

    #[test]
    fn test_safe_extend() {
        let s = MirrorFoldSystem::new();
        assert!(s.safe_to_extend());
    }

    #[test]
    fn test_no_drag_extended() {
        let s = MirrorFoldSystem::new();
        assert!((s.drag_reduction_when_folded() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_drag_folded() {
        let mut s = MirrorFoldSystem::new();
        s.left_state = MirrorState::Folded;
        s.right_state = MirrorState::Folded;
        assert!((s.drag_reduction_when_folded() - 0.02).abs() < 0.001);
    }
}
