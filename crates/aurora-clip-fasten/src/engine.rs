/// Clip fastener: push-in, screw-in, quarter-turn, trim clips
/// Phase 407

#[derive(Debug, Clone)]
pub struct ClipFasten {
    pub installed_count: u32,
    pub required_count: u32,
    pub broken_count: u32,
    pub correct_type: bool,
    pub secure: bool,
}

impl Default for ClipFasten {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipFasten {
    pub fn new() -> Self {
        Self {
            installed_count: 20,
            required_count: 20,
            broken_count: 0,
            correct_type: true,
            secure: true,
        }
    }

    pub fn all_present(&self) -> bool {
        self.installed_count >= self.required_count
    }

    pub fn all_ok(&self) -> bool {
        self.all_present() && self.broken_count == 0 && self.secure
    }

    pub fn missing_count(&self) -> u32 {
        self.required_count.saturating_sub(self.installed_count)
    }

    pub fn needs_service(&self) -> bool {
        self.broken_count > 0 || !self.all_present()
    }

    pub fn health_score(&self) -> f64 {
        if !self.all_present() {
            return 30.0;
        }
        if self.broken_count > 0 {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present() {
        let c = ClipFasten::new();
        assert!(c.all_present());
    }

    #[test]
    fn test_all_ok() {
        let c = ClipFasten::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_none_missing() {
        let c = ClipFasten::new();
        assert_eq!(c.missing_count(), 0);
    }

    #[test]
    fn test_no_service() {
        let c = ClipFasten::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_broken() {
        let mut c = ClipFasten::new();
        c.broken_count = 3;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ClipFasten::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
