//! Lane keeping assistance — automatic steering corrections to maintain lane position.

/// Lane keeping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneKeepMode {
    /// Lane keeping is off.
    Off,
    /// Warning only — alert driver but no steering intervention.
    WarningOnly,
    /// Assist — gentle steering corrections.
    Assist,
    /// Active — full lane centering.
    Active,
}

/// Lane keeping steering command.
#[derive(Debug, Clone, Copy)]
pub struct SteeringCommand {
    /// Steering torque (-1.0 = full left, 1.0 = full right).
    pub torque: f64,
    /// Confidence in the command (0.0 to 1.0).
    pub confidence: f64,
    /// Whether this is an override (driver should still be in control).
    pub is_override: bool,
}

/// Lane detection quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneQuality {
    /// Both lane markings clearly visible.
    Good,
    /// One lane marking visible.
    Partial,
    /// Lane markings unclear or absent.
    Poor,
    /// No lane markings detected.
    None,
}

/// Configuration for lane keeping.
#[derive(Debug, Clone)]
pub struct LaneKeepConfig {
    /// Maximum steering torque in assist mode.
    pub max_assist_torque: f64,
    /// Maximum steering torque in active mode.
    pub max_active_torque: f64,
    /// Proportional gain for lane centering.
    pub proportional_gain: f64,
    /// Derivative gain for steering smoothness.
    pub derivative_gain: f64,
    /// Minimum lane quality for active mode.
    pub min_quality_active: LaneQuality,
    /// Minimum speed for lane keeping (m/s).
    pub min_speed: f64,
}

impl Default for LaneKeepConfig {
    fn default() -> Self {
        Self {
            max_assist_torque: 0.3,
            max_active_torque: 0.6,
            proportional_gain: 0.5,
            derivative_gain: 0.2,
            min_quality_active: LaneQuality::Partial,
            min_speed: 16.67, // ~60 km/h
        }
    }
}

/// Lane keeping assistance engine.
pub struct LaneKeepAssist {
    config: LaneKeepConfig,
    mode: LaneKeepMode,
    last_deviation: f64,
    intervention_count: u64,
}

impl LaneKeepAssist {
    /// Create a new lane keep assist.
    pub fn new(config: LaneKeepConfig) -> Self {
        Self {
            config,
            mode: LaneKeepMode::Off,
            last_deviation: 0.0,
            intervention_count: 0,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(LaneKeepConfig::default())
    }

    /// Set the lane keeping mode.
    pub fn set_mode(&mut self, mode: LaneKeepMode) {
        self.mode = mode;
    }

    /// Get current mode.
    pub fn mode(&self) -> LaneKeepMode {
        self.mode
    }

    /// Get the number of steering interventions made.
    pub fn intervention_count(&self) -> u64 {
        self.intervention_count
    }

    /// Calculate steering correction based on lane position.
    ///
    /// - `deviation_m`: lateral deviation from lane centre (positive = right).
    /// - `speed_mps`: current vehicle speed (m/s).
    /// - `lane_quality`: quality of lane marking detection.
    pub fn calculate_correction(
        &mut self,
        deviation_m: f64,
        speed_mps: f64,
        lane_quality: LaneQuality,
    ) -> SteeringCommand {
        // Off mode — no correction
        if self.mode == LaneKeepMode::Off {
            self.last_deviation = deviation_m;
            return SteeringCommand {
                torque: 0.0,
                confidence: 0.0,
                is_override: false,
            };
        }

        // Below minimum speed — no correction
        if speed_mps < self.config.min_speed {
            self.last_deviation = deviation_m;
            return SteeringCommand {
                torque: 0.0,
                confidence: 0.0,
                is_override: false,
            };
        }

        // Lane quality check for active mode
        if self.mode == LaneKeepMode::Active && !self.quality_sufficient(lane_quality) {
            self.last_deviation = deviation_m;
            return SteeringCommand {
                torque: 0.0,
                confidence: 0.0,
                is_override: false,
            };
        }

        // PD controller for steering
        let derivative = deviation_m - self.last_deviation;
        let raw_torque = -(self.config.proportional_gain * deviation_m
            + self.config.derivative_gain * derivative);

        let max_torque = match self.mode {
            LaneKeepMode::Assist => self.config.max_assist_torque,
            LaneKeepMode::Active => self.config.max_active_torque,
            _ => 0.0,
        };

        let clamped_torque = raw_torque.clamp(-max_torque, max_torque);
        let confidence = self.quality_confidence(lane_quality);

        self.last_deviation = deviation_m;
        if clamped_torque.abs() > 0.01 {
            self.intervention_count += 1;
        }

        SteeringCommand {
            torque: clamped_torque,
            confidence,
            is_override: self.mode == LaneKeepMode::WarningOnly,
        }
    }

    fn quality_sufficient(&self, quality: LaneQuality) -> bool {
        match self.config.min_quality_active {
            LaneQuality::Good => quality == LaneQuality::Good,
            LaneQuality::Partial => quality == LaneQuality::Good || quality == LaneQuality::Partial,
            LaneQuality::Poor => quality != LaneQuality::None,
            LaneQuality::None => true,
        }
    }

    fn quality_confidence(&self, quality: LaneQuality) -> f64 {
        match quality {
            LaneQuality::Good => 1.0,
            LaneQuality::Partial => 0.7,
            LaneQuality::Poor => 0.3,
            LaneQuality::None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_off_no_correction() {
        let mut lka = LaneKeepAssist::with_defaults();
        let cmd = lka.calculate_correction(1.0, 30.0, LaneQuality::Good);
        assert_eq!(cmd.torque, 0.0);
    }

    #[test]
    fn test_assist_corrects_right_deviation() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        // Deviated 0.5m to the right — should steer left (negative torque)
        let cmd = lka.calculate_correction(0.5, 30.0, LaneQuality::Good);
        assert!(
            cmd.torque < 0.0,
            "Should steer left for right deviation, got {}",
            cmd.torque
        );
    }

    #[test]
    fn test_assist_corrects_left_deviation() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        // Deviated -0.5m to the left — should steer right (positive torque)
        let cmd = lka.calculate_correction(-0.5, 30.0, LaneQuality::Good);
        assert!(
            cmd.torque > 0.0,
            "Should steer right for left deviation, got {}",
            cmd.torque
        );
    }

    #[test]
    fn test_torque_clamped_in_assist() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        let cmd = lka.calculate_correction(5.0, 30.0, LaneQuality::Good);
        assert!(
            cmd.torque.abs() <= 0.3,
            "Assist torque should be clamped to 0.3"
        );
    }

    #[test]
    fn test_active_mode_higher_torque_limit() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Active);
        let cmd = lka.calculate_correction(5.0, 30.0, LaneQuality::Good);
        assert!(
            cmd.torque.abs() <= 0.6,
            "Active torque should be clamped to 0.6"
        );
        assert!(
            cmd.torque.abs() > 0.3,
            "Active torque should exceed assist limit for large deviation"
        );
    }

    #[test]
    fn test_no_correction_below_min_speed() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Active);
        let cmd = lka.calculate_correction(1.0, 10.0, LaneQuality::Good); // Below 16.67 m/s
        assert_eq!(cmd.torque, 0.0);
    }

    #[test]
    fn test_active_disabled_with_no_lanes() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Active);
        let cmd = lka.calculate_correction(1.0, 30.0, LaneQuality::None);
        assert_eq!(cmd.torque, 0.0);
    }

    #[test]
    fn test_confidence_by_quality() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        let good = lka.calculate_correction(0.3, 30.0, LaneQuality::Good);
        assert_eq!(good.confidence, 1.0);
        let partial = lka.calculate_correction(0.3, 30.0, LaneQuality::Partial);
        assert!((partial.confidence - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_centered_no_intervention() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        let cmd = lka.calculate_correction(0.0, 30.0, LaneQuality::Good);
        assert!(cmd.torque.abs() < 0.01);
    }

    #[test]
    fn test_intervention_count() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::Assist);
        lka.calculate_correction(0.5, 30.0, LaneQuality::Good);
        lka.calculate_correction(0.3, 30.0, LaneQuality::Good);
        assert!(lka.intervention_count() >= 1);
    }

    #[test]
    fn test_warning_mode_is_override() {
        let mut lka = LaneKeepAssist::with_defaults();
        lka.set_mode(LaneKeepMode::WarningOnly);
        let cmd = lka.calculate_correction(0.5, 30.0, LaneQuality::Good);
        assert!(cmd.is_override);
    }
}
