/// Axle seal: inner, outer, material, preload
/// Phase 815

#[derive(Debug, Clone)]
pub struct AxleSeal {
    pub inner_ok: bool,
    pub outer_ok: bool,
    pub material_ok: bool,
    pub preload_ok: bool,
    pub leak_free: bool,
}

impl Default for AxleSeal {
    fn default() -> Self {
        Self::new()
    }
}

impl AxleSeal {
    pub fn new() -> Self {
        Self {
            inner_ok: true,
            outer_ok: true,
            material_ok: true,
            preload_ok: true,
            leak_free: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.inner_ok && self.outer_ok && self.leak_free
    }

    pub fn condition_ok(&self) -> bool {
        self.material_ok && self.preload_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.leak_free || !self.inner_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = AxleSeal::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_condition() {
        let c = AxleSeal::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AxleSeal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = AxleSeal::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut c = AxleSeal::new();
        c.leak_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = AxleSeal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
