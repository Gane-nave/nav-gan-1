/// Chrome trim: plating, polish, corrosion, adhesion
/// Phase 773

#[derive(Debug, Clone)]
pub struct ChromeTrim {
    pub plating_ok: bool,
    pub polish_ok: bool,
    pub corrosion_free: bool,
    pub adhesion_ok: bool,
    pub alignment_ok: bool,
}

impl Default for ChromeTrim {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeTrim {
    pub fn new() -> Self {
        Self {
            plating_ok: true,
            polish_ok: true,
            corrosion_free: true,
            adhesion_ok: true,
            alignment_ok: true,
        }
    }

    pub fn finish_ok(&self) -> bool {
        self.plating_ok && self.polish_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.corrosion_free && self.adhesion_ok && self.alignment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.finish_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.plating_ok || !self.corrosion_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.plating_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish() {
        let c = ChromeTrim::new();
        assert!(c.finish_ok());
    }

    #[test]
    fn test_condition() {
        let c = ChromeTrim::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChromeTrim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ChromeTrim::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_plating() {
        let mut c = ChromeTrim::new();
        c.plating_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ChromeTrim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
