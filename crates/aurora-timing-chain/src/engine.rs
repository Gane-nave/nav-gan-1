/// Timing chain: stretch, tensioner, guide, sprocket
/// Phase 576

#[derive(Debug, Clone)]
pub struct TimingChain {
    pub stretch_mm: f64,
    pub max_stretch_mm: f64,
    pub tensioner_ok: bool,
    pub guide_ok: bool,
    pub sprocket_ok: bool,
}

impl Default for TimingChain {
    fn default() -> Self {
        Self::new()
    }
}

impl TimingChain {
    pub fn new() -> Self {
        Self {
            stretch_mm: 0.5,
            max_stretch_mm: 3.0,
            tensioner_ok: true,
            guide_ok: true,
            sprocket_ok: true,
        }
    }

    pub fn stretch_ok(&self) -> bool {
        self.stretch_mm < self.max_stretch_mm
    }

    pub fn components_ok(&self) -> bool {
        self.tensioner_ok && self.guide_ok && self.sprocket_ok
    }

    pub fn all_ok(&self) -> bool {
        self.stretch_ok() && self.components_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.stretch_mm > self.max_stretch_mm || !self.guide_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.stretch_mm > self.max_stretch_mm { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stretch() {
        let c = TimingChain::new();
        assert!(c.stretch_ok());
    }

    #[test]
    fn test_components() {
        let c = TimingChain::new();
        assert!(c.components_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TimingChain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TimingChain::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_stretched() {
        let mut c = TimingChain::new();
        c.stretch_mm = 4.0;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TimingChain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
