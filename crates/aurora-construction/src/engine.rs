/// Construction zone: detection, reroute, delay, merge
/// Phase 920

#[derive(Debug, Clone)]
pub struct Construction {
    pub detect_ok: bool,
    pub reroute_ok: bool,
    pub delay_ok: bool,
    pub merge_ok: bool,
    pub update_ok: bool,
}

impl Default for Construction {
    fn default() -> Self {
        Self::new()
    }
}

impl Construction {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            reroute_ok: true,
            delay_ok: true,
            merge_ok: true,
            update_ok: true,
        }
    }

    pub fn awareness_ok(&self) -> bool {
        self.detect_ok && self.delay_ok && self.update_ok
    }

    pub fn navigation_ok(&self) -> bool {
        self.reroute_ok && self.merge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.awareness_ok() && self.navigation_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.update_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awareness() {
        let c = Construction::new();
        assert!(c.awareness_ok());
    }

    #[test]
    fn test_navigation() {
        let c = Construction::new();
        assert!(c.navigation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Construction::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Construction::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_update() {
        let mut c = Construction::new();
        c.update_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Construction::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
