/// Launch control: RPM limiting, traction management, launch optimization
/// Phase 208

#[derive(Debug, Clone)]
pub struct LaunchControl {
    pub armed: bool,
    pub launch_rpm: f64,
    pub current_rpm: f64,
    pub traction_limit_pct: f64,
    pub launches_count: u32,
    pub enabled: bool,
}

impl Default for LaunchControl {
    fn default() -> Self {
        Self::new()
    }
}

impl LaunchControl {
    pub fn new() -> Self {
        Self {
            armed: false,
            launch_rpm: 4000.0,
            current_rpm: 800.0,
            traction_limit_pct: 100.0,
            launches_count: 0,
            enabled: true,
        }
    }

    pub fn ready_to_launch(&self) -> bool {
        self.armed && self.enabled && (self.current_rpm - self.launch_rpm).abs() < 200.0
    }

    pub fn rpm_at_target(&self) -> bool {
        (self.current_rpm - self.launch_rpm).abs() < 200.0
    }

    pub fn traction_limited(&self) -> bool {
        self.traction_limit_pct < 100.0
    }

    pub fn over_revving(&self) -> bool {
        self.current_rpm > self.launch_rpm * 1.1
    }

    pub fn health_score(&self) -> f64 {
        if !self.enabled {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_ready() {
        let l = LaunchControl::new();
        assert!(!l.ready_to_launch());
    }

    #[test]
    fn test_not_at_target() {
        let l = LaunchControl::new();
        assert!(!l.rpm_at_target());
    }

    #[test]
    fn test_no_traction_limit() {
        let l = LaunchControl::new();
        assert!(!l.traction_limited());
    }

    #[test]
    fn test_not_over_revving() {
        let l = LaunchControl::new();
        assert!(!l.over_revving());
    }

    #[test]
    fn test_ready() {
        let mut l = LaunchControl::new();
        l.armed = true;
        l.current_rpm = 4000.0;
        assert!(l.ready_to_launch());
    }

    #[test]
    fn test_health() {
        let l = LaunchControl::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
