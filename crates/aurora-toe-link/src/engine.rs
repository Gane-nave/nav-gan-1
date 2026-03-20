/// Toe link: adjustment, bushing, ball end, lock nut
/// Phase 653

#[derive(Debug, Clone)]
pub struct ToeLink {
    pub adjustment_ok: bool,
    pub bushing_ok: bool,
    pub ball_end_ok: bool,
    pub lock_nut_ok: bool,
    pub aligned: bool,
}

impl Default for ToeLink {
    fn default() -> Self {
        Self::new()
    }
}

impl ToeLink {
    pub fn new() -> Self {
        Self {
            adjustment_ok: true,
            bushing_ok: true,
            ball_end_ok: true,
            lock_nut_ok: true,
            aligned: true,
        }
    }

    pub fn link_ok(&self) -> bool {
        self.bushing_ok && self.ball_end_ok
    }

    pub fn setting_ok(&self) -> bool {
        self.adjustment_ok && self.lock_nut_ok && self.aligned
    }

    pub fn all_ok(&self) -> bool {
        self.link_ok() && self.setting_ok()
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.aligned || !self.lock_nut_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ball_end_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link() {
        let c = ToeLink::new();
        assert!(c.link_ok());
    }

    #[test]
    fn test_setting() {
        let c = ToeLink::new();
        assert!(c.setting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ToeLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = ToeLink::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_align() {
        let mut c = ToeLink::new();
        c.aligned = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = ToeLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
