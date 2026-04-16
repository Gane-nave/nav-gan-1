/// Scratch detection: depth measurement, paint penetration, repairability
/// Phase 391

#[derive(Debug, Clone)]
pub struct ScratchDet {
    pub depth_um: f64,
    pub length_mm: f64,
    pub through_clear: bool,
    pub through_base: bool,
    pub count: u32,
}

impl Default for ScratchDet {
    fn default() -> Self {
        Self::new()
    }
}

impl ScratchDet {
    pub fn new() -> Self {
        Self {
            depth_um: 5.0,
            length_mm: 20.0,
            through_clear: false,
            through_base: false,
            count: 2,
        }
    }

    pub fn superficial(&self) -> bool {
        !self.through_clear && self.depth_um < 15.0
    }

    pub fn repairable(&self) -> bool {
        !self.through_base
    }

    pub fn needs_repaint(&self) -> bool {
        self.through_base
    }

    pub fn severity(&self) -> &str {
        if self.through_base {
            "severe"
        } else if self.through_clear {
            "moderate"
        } else {
            "light"
        }
    }

    pub fn health_score(&self) -> f64 {
        if self.through_base {
            return 10.0;
        }
        if self.through_clear {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superficial() {
        let s = ScratchDet::new();
        assert!(s.superficial());
    }

    #[test]
    fn test_repairable() {
        let s = ScratchDet::new();
        assert!(s.repairable());
    }

    #[test]
    fn test_no_repaint() {
        let s = ScratchDet::new();
        assert!(!s.needs_repaint());
    }

    #[test]
    fn test_severity() {
        let s = ScratchDet::new();
        assert_eq!(s.severity(), "light");
    }

    #[test]
    fn test_deep() {
        let mut s = ScratchDet::new();
        s.through_base = true;
        assert!(s.needs_repaint());
    }

    #[test]
    fn test_health() {
        let s = ScratchDet::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
