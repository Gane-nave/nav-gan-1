/// store bitmap: set, get, count, clear, log
/// Phase 1985

#[derive(Debug, Clone)]
pub struct StoreBitmap {
    pub set_ok: bool,
    pub get_ok: bool,
    pub count_ok: bool,
    pub clear_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreBitmap {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreBitmap {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            get_ok: true,
            count_ok: true,
            clear_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.get_ok && self.count_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = StoreBitmap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreBitmap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreBitmap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreBitmap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreBitmap::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreBitmap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
