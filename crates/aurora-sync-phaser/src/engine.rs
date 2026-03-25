/// sync phaser: arrive, advance, await_adv, terminate, log
/// Phase 2389

#[derive(Debug, Clone)]
pub struct SyncPhaser {
    pub arrive_ok: bool,
    pub advance_ok: bool,
    pub await_adv_ok: bool,
    pub terminate_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncPhaser {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncPhaser {
    pub fn new() -> Self {
        Self {
            arrive_ok: true,
            advance_ok: true,
            await_adv_ok: true,
            terminate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.arrive_ok && self.advance_ok && self.await_adv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.terminate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.arrive_ok || !self.advance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.arrive_ok {
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
        let c = SyncPhaser::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncPhaser::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncPhaser::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncPhaser::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncPhaser::new();
        c.arrive_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncPhaser::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
