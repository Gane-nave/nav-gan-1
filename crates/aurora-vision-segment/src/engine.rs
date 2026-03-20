/// vision segment: capture, preprocess, segment, mask, log
/// Phase 1475

#[derive(Debug, Clone)]
pub struct VisionSegment {
    pub capture_ok: bool,
    pub preprocess_ok: bool,
    pub segment_ok: bool,
    pub mask_ok: bool,
    pub log_ok: bool,
}

impl Default for VisionSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl VisionSegment {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            preprocess_ok: true,
            segment_ok: true,
            mask_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.preprocess_ok && self.segment_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.mask_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.preprocess_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VisionSegment::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VisionSegment::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VisionSegment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VisionSegment::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VisionSegment::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VisionSegment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
