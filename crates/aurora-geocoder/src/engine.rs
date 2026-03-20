/// Geocoder: forward, reverse, autocomplete, batch, validate
/// Phase 1090

#[derive(Debug, Clone)]
pub struct Geocoder {
    pub forward_ok: bool,
    pub reverse_ok: bool,
    pub autocomplete_ok: bool,
    pub batch_ok: bool,
    pub validate_ok: bool,
}

impl Default for Geocoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Geocoder {
    pub fn new() -> Self {
        Self {
            forward_ok: true,
            reverse_ok: true,
            autocomplete_ok: true,
            batch_ok: true,
            validate_ok: true,
        }
    }

    pub fn coding_ok(&self) -> bool {
        self.forward_ok && self.reverse_ok && self.autocomplete_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.batch_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.coding_ok() && self.processing_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.forward_ok || !self.reverse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.forward_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coding() {
        let c = Geocoder::new();
        assert!(c.coding_ok());
    }

    #[test]
    fn test_processing() {
        let c = Geocoder::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Geocoder::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = Geocoder::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_forward() {
        let mut c = Geocoder::new();
        c.forward_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = Geocoder::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
