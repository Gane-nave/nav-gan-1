//! Adaptive cruise control — speed management with traffic-aware following distance.

/// Cruise control mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CruiseMode {
    /// Cruise control is off.
    Off,
    /// Standard cruise — maintain set speed.
    Standard,
    /// Adaptive — adjust speed to maintain following distance.
    Adaptive,
    /// Eco — prioritise fuel efficiency.
    Eco,
    /// Sport — allow closer following, faster acceleration.
    Sport,
}

/// Cruise control action command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CruiseAction {
    /// Maintain current speed.
    Maintain,
    /// Accelerate to target speed.
    Accelerate,
    /// Decelerate to match traffic.
    Decelerate,
    /// Emergency stop — obstacle too close.
    EmergencyStop,
    /// Disengage cruise control.
    Disengage,
}

/// Cruise control configuration.
#[derive(Debug, Clone)]
pub struct CruiseConfig {
    /// Minimum following time gap (seconds).
    pub min_time_gap: f64,
    /// Target following time gap (seconds).
    pub target_time_gap: f64,
    /// Maximum acceleration (m/s²).
    pub max_acceleration: f64,
    /// Maximum deceleration (m/s²).
    pub max_deceleration: f64,
    /// Emergency deceleration (m/s²).
    pub emergency_deceleration: f64,
    /// Speed tolerance for maintaining speed (m/s).
    pub speed_tolerance: f64,
    /// Minimum speed for cruise to be active (m/s).
    pub min_active_speed: f64,
}

impl Default for CruiseConfig {
    fn default() -> Self {
        Self {
            min_time_gap: 1.0,
            target_time_gap: 2.0,
            max_acceleration: 2.0,
            max_deceleration: 3.0,
            emergency_deceleration: 8.0,
            speed_tolerance: 0.5,
            min_active_speed: 8.33, // ~30 km/h
        }
    }
}

/// Adaptive cruise control engine.
pub struct CruiseController {
    config: CruiseConfig,
    mode: CruiseMode,
    target_speed: f64,
    current_speed: f64,
}

impl CruiseController {
    /// Create a new cruise controller.
    pub fn new(config: CruiseConfig) -> Self {
        Self {
            config,
            mode: CruiseMode::Off,
            target_speed: 0.0,
            current_speed: 0.0,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(CruiseConfig::default())
    }

    /// Engage cruise control at the current speed.
    pub fn engage(&mut self, mode: CruiseMode, current_speed: f64) -> bool {
        if mode == CruiseMode::Off {
            self.disengage();
            return false;
        }

        if current_speed < self.config.min_active_speed {
            return false; // Too slow
        }

        self.mode = mode;
        self.target_speed = current_speed;
        self.current_speed = current_speed;
        true
    }

    /// Disengage cruise control.
    pub fn disengage(&mut self) {
        self.mode = CruiseMode::Off;
    }

    /// Set the target speed.
    pub fn set_target_speed(&mut self, speed: f64) {
        self.target_speed = speed.max(0.0);
    }

    /// Get current mode.
    pub fn mode(&self) -> CruiseMode {
        self.mode
    }

    /// Get the target speed.
    pub fn target_speed(&self) -> f64 {
        self.target_speed
    }

    /// Update with current traffic conditions and get the cruise action.
    pub fn update(
        &mut self,
        current_speed: f64,
        lead_vehicle_distance: Option<f64>,
        lead_vehicle_speed: Option<f64>,
    ) -> CruiseAction {
        self.current_speed = current_speed;

        if self.mode == CruiseMode::Off {
            return CruiseAction::Disengage;
        }

        // Speed fell below minimum — disengage
        if current_speed < self.config.min_active_speed
            && self.target_speed > self.config.min_active_speed
        {
            self.disengage();
            return CruiseAction::Disengage;
        }

        // Check lead vehicle in adaptive mode
        if self.mode == CruiseMode::Adaptive
            || self.mode == CruiseMode::Eco
            || self.mode == CruiseMode::Sport
        {
            if let (Some(distance), Some(lead_speed)) = (lead_vehicle_distance, lead_vehicle_speed)
            {
                return self.adaptive_action(current_speed, distance, lead_speed);
            }
        }

        // Standard cruise — match target speed
        self.speed_match_action(current_speed, self.target_speed)
    }

    /// Calculate required following distance based on mode.
    pub fn required_following_distance(&self, speed: f64) -> f64 {
        let time_gap = match self.mode {
            CruiseMode::Sport => self.config.min_time_gap,
            CruiseMode::Eco => self.config.target_time_gap * 1.5,
            _ => self.config.target_time_gap,
        };
        speed * time_gap
    }

    fn adaptive_action(&self, current_speed: f64, distance: f64, lead_speed: f64) -> CruiseAction {
        let time_gap = if current_speed > 0.0 {
            distance / current_speed
        } else {
            f64::MAX
        };

        // Emergency: too close
        if time_gap < self.config.min_time_gap * 0.5 {
            return CruiseAction::EmergencyStop;
        }

        let target_gap = match self.mode {
            CruiseMode::Sport => self.config.min_time_gap,
            CruiseMode::Eco => self.config.target_time_gap * 1.5,
            _ => self.config.target_time_gap,
        };

        if time_gap < target_gap {
            // Too close — match lead vehicle speed or decelerate
            if lead_speed < current_speed {
                CruiseAction::Decelerate
            } else {
                CruiseAction::Maintain
            }
        } else {
            // Adequate gap — aim for target speed
            self.speed_match_action(current_speed, self.target_speed.min(lead_speed + 5.0))
        }
    }

    fn speed_match_action(&self, current: f64, target: f64) -> CruiseAction {
        let diff = target - current;
        if diff.abs() < self.config.speed_tolerance {
            CruiseAction::Maintain
        } else if diff > 0.0 {
            CruiseAction::Accelerate
        } else {
            CruiseAction::Decelerate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_off() {
        let ctrl = CruiseController::with_defaults();
        assert_eq!(ctrl.mode(), CruiseMode::Off);
    }

    #[test]
    fn test_engage_at_speed() {
        let mut ctrl = CruiseController::with_defaults();
        assert!(ctrl.engage(CruiseMode::Adaptive, 30.0)); // 30 m/s = ~108 km/h
        assert_eq!(ctrl.mode(), CruiseMode::Adaptive);
        assert_eq!(ctrl.target_speed(), 30.0);
    }

    #[test]
    fn test_engage_fails_below_min_speed() {
        let mut ctrl = CruiseController::with_defaults();
        assert!(!ctrl.engage(CruiseMode::Standard, 5.0)); // Too slow
        assert_eq!(ctrl.mode(), CruiseMode::Off);
    }

    #[test]
    fn test_standard_cruise_maintain() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Standard, 30.0);
        let action = ctrl.update(30.0, None, None);
        assert_eq!(action, CruiseAction::Maintain);
    }

    #[test]
    fn test_standard_cruise_accelerate() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Standard, 30.0);
        let action = ctrl.update(25.0, None, None); // Below target
        assert_eq!(action, CruiseAction::Accelerate);
    }

    #[test]
    fn test_standard_cruise_decelerate() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Standard, 30.0);
        let action = ctrl.update(35.0, None, None); // Above target
        assert_eq!(action, CruiseAction::Decelerate);
    }

    #[test]
    fn test_adaptive_decelerate_for_lead() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Adaptive, 30.0);
        // Lead vehicle at 20m doing 20m/s — time gap = 20/30 = 0.67s (below target 2.0s)
        let action = ctrl.update(30.0, Some(20.0), Some(20.0));
        assert_eq!(action, CruiseAction::Decelerate);
    }

    #[test]
    fn test_adaptive_emergency_stop() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Adaptive, 30.0);
        // Lead vehicle at 5m — time gap = 5/30 = 0.17s (below min_time_gap * 0.5 = 0.5s)
        let action = ctrl.update(30.0, Some(5.0), Some(10.0));
        assert_eq!(action, CruiseAction::EmergencyStop);
    }

    #[test]
    fn test_disengage() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Standard, 30.0);
        ctrl.disengage();
        assert_eq!(ctrl.mode(), CruiseMode::Off);
    }

    #[test]
    fn test_off_returns_disengage() {
        let mut ctrl = CruiseController::with_defaults();
        let action = ctrl.update(30.0, None, None);
        assert_eq!(action, CruiseAction::Disengage);
    }

    #[test]
    fn test_eco_mode_larger_gap() {
        let ctrl_eco = {
            let mut c = CruiseController::with_defaults();
            c.engage(CruiseMode::Eco, 30.0);
            c
        };
        let ctrl_sport = {
            let mut c = CruiseController::with_defaults();
            c.engage(CruiseMode::Sport, 30.0);
            c
        };
        let eco_dist = ctrl_eco.required_following_distance(30.0);
        let sport_dist = ctrl_sport.required_following_distance(30.0);
        assert!(
            eco_dist > sport_dist,
            "Eco gap ({eco_dist}) should be larger than Sport gap ({sport_dist})"
        );
    }

    #[test]
    fn test_set_target_speed() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.engage(CruiseMode::Standard, 30.0);
        ctrl.set_target_speed(35.0);
        assert_eq!(ctrl.target_speed(), 35.0);
    }

    #[test]
    fn test_negative_target_speed_clamped() {
        let mut ctrl = CruiseController::with_defaults();
        ctrl.set_target_speed(-10.0);
        assert_eq!(ctrl.target_speed(), 0.0);
    }
}
