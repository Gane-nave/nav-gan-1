/// FlexRay controller: deterministic communication, dual-channel, safety-critical
/// Phase 279

#[derive(Debug, Clone)]
pub struct FlexRayController {
    pub channel_a_ok: bool,
    pub channel_b_ok: bool,
    pub cycle_time_us: u32,
    pub sync_ok: bool,
    pub slot_count: u16,
    pub error_count: u32,
}

impl Default for FlexRayController {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexRayController {
    pub fn new() -> Self {
        Self {
            channel_a_ok: true,
            channel_b_ok: true,
            cycle_time_us: 5000,
            sync_ok: true,
            slot_count: 64,
            error_count: 0,
        }
    }

    pub fn dual_channel(&self) -> bool {
        self.channel_a_ok && self.channel_b_ok
    }

    pub fn any_channel(&self) -> bool {
        self.channel_a_ok || self.channel_b_ok
    }

    pub fn synchronized(&self) -> bool {
        self.sync_ok && self.any_channel()
    }

    pub fn error_free(&self) -> bool {
        self.error_count == 0
    }

    pub fn health_score(&self) -> f64 {
        if !self.any_channel() {
            return 0.0;
        }
        if !self.dual_channel() {
            return 50.0;
        }
        if !self.sync_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual() {
        let f = FlexRayController::new();
        assert!(f.dual_channel());
    }

    #[test]
    fn test_any() {
        let f = FlexRayController::new();
        assert!(f.any_channel());
    }

    #[test]
    fn test_sync() {
        let f = FlexRayController::new();
        assert!(f.synchronized());
    }

    #[test]
    fn test_error_free() {
        let f = FlexRayController::new();
        assert!(f.error_free());
    }

    #[test]
    fn test_single_channel() {
        let mut f = FlexRayController::new();
        f.channel_b_ok = false;
        assert!(!f.dual_channel());
    }

    #[test]
    fn test_health() {
        let f = FlexRayController::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
