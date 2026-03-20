/// Tie rod: steering linkage, inner/outer ends
/// Phase 479

#[derive(Debug, Clone)]
pub struct TieRod {
    pub play_mm: f64,
    pub max_play_mm: f64,
    pub inner_ok: bool,
    pub outer_ok: bool,
    pub boot_ok: bool,
}

impl Default for TieRod {
    fn default() -> Self {
        Self::new()
    }
}

impl TieRod {
    pub fn new() -> Self {
        Self {
            play_mm: 0.3,
            max_play_mm: 2.0,
            inner_ok: true,
            outer_ok: true,
            boot_ok: true,
        }
    }

    pub fn play_pct(&self) -> f64 {
        (self.play_mm / self.max_play_mm) * 100.0
    }

    pub fn excessive_play(&self) -> bool {
        self.play_mm > self.max_play_mm * 0.8
    }

    pub fn all_ok(&self) -> bool {
        self.inner_ok && self.outer_ok && self.boot_ok && !self.excessive_play()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.inner_ok || !self.outer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inner_ok || !self.outer_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let c = TieRod::new();
        assert!(c.play_pct() < 25.0);
    }

    #[test]
    fn test_no_excessive() {
        let c = TieRod::new();
        assert!(!c.excessive_play());
    }

    #[test]
    fn test_all_ok() {
        let c = TieRod::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TieRod::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_inner_bad() {
        let mut c = TieRod::new();
        c.inner_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TieRod::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
