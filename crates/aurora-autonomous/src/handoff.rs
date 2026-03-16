//! Human-machine handoff protocols — safe transitions between autonomous and manual driving.

use std::time::{Duration, Instant};

/// Handoff direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffDirection {
    /// System to human (autonomous → manual).
    ToHuman,
    /// Human to system (manual → autonomous).
    ToSystem,
}

/// Handoff state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffState {
    /// No handoff in progress.
    Idle,
    /// Handoff initiated — waiting for acknowledgement.
    Initiated,
    /// Handoff in progress — transitioning control.
    InProgress,
    /// Handoff completed successfully.
    Completed,
    /// Handoff failed — fallback mode.
    Failed,
    /// Handoff timed out.
    TimedOut,
}

/// Handoff urgency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandoffUrgency {
    /// Planned handoff — plenty of time.
    Planned,
    /// Prompt — system needs handoff soon.
    Prompt,
    /// Urgent — immediate handoff required.
    Urgent,
    /// Emergency — system cannot continue safely.
    Emergency,
}

/// Reason for handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffReason {
    /// Driver requested manual control.
    DriverRequested,
    /// System capability limit reached.
    SystemLimit,
    /// Road conditions require human judgement.
    RoadConditions,
    /// Sensor degradation.
    SensorDegraded,
    /// Approaching end of autonomous-capable zone.
    ZoneBoundary,
    /// Emergency situation.
    Emergency,
}

/// Configuration for handoff protocols.
#[derive(Debug, Clone)]
pub struct HandoffConfig {
    /// Maximum time allowed for driver to acknowledge (seconds).
    pub acknowledgement_timeout: Duration,
    /// Transition period for gradual handoff (seconds).
    pub transition_period: Duration,
    /// Number of escalation stages before timeout.
    pub escalation_stages: u32,
    /// Whether to allow handoff at high speed.
    pub allow_high_speed_handoff: bool,
    /// High speed threshold (m/s).
    pub high_speed_threshold: f64,
}

impl Default for HandoffConfig {
    fn default() -> Self {
        Self {
            acknowledgement_timeout: Duration::from_secs(15),
            transition_period: Duration::from_secs(5),
            escalation_stages: 3,
            allow_high_speed_handoff: true,
            high_speed_threshold: 33.33, // ~120 km/h
        }
    }
}

/// Handoff protocol manager.
pub struct HandoffManager {
    config: HandoffConfig,
    state: HandoffState,
    direction: Option<HandoffDirection>,
    urgency: Option<HandoffUrgency>,
    reason: Option<HandoffReason>,
    initiated_at: Option<Instant>,
    escalation_level: u32,
    completed_handoffs: u32,
    failed_handoffs: u32,
}

impl HandoffManager {
    /// Create a new handoff manager.
    pub fn new(config: HandoffConfig) -> Self {
        Self {
            config,
            state: HandoffState::Idle,
            direction: None,
            urgency: None,
            reason: None,
            initiated_at: None,
            escalation_level: 0,
            completed_handoffs: 0,
            failed_handoffs: 0,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(HandoffConfig::default())
    }

    /// Get the current handoff state.
    pub fn state(&self) -> HandoffState {
        self.state
    }

    /// Get the current escalation level.
    pub fn escalation_level(&self) -> u32 {
        self.escalation_level
    }

    /// Get completed handoff count.
    pub fn completed_count(&self) -> u32 {
        self.completed_handoffs
    }

    /// Get failed handoff count.
    pub fn failed_count(&self) -> u32 {
        self.failed_handoffs
    }

    /// Initiate a handoff.
    pub fn initiate(
        &mut self,
        direction: HandoffDirection,
        urgency: HandoffUrgency,
        reason: HandoffReason,
        current_speed: f64,
    ) -> Result<(), HandoffError> {
        if self.state != HandoffState::Idle
            && self.state != HandoffState::Completed
            && self.state != HandoffState::Failed
            && self.state != HandoffState::TimedOut
        {
            return Err(HandoffError::AlreadyInProgress);
        }

        if !self.config.allow_high_speed_handoff && current_speed > self.config.high_speed_threshold
        {
            return Err(HandoffError::SpeedTooHigh);
        }

        self.state = HandoffState::Initiated;
        self.direction = Some(direction);
        self.urgency = Some(urgency);
        self.reason = Some(reason);
        self.initiated_at = Some(Instant::now());
        self.escalation_level = 0;

        Ok(())
    }

    /// Driver acknowledges the handoff request.
    pub fn acknowledge(&mut self) -> Result<(), HandoffError> {
        if self.state != HandoffState::Initiated {
            return Err(HandoffError::InvalidState);
        }

        self.state = HandoffState::InProgress;
        Ok(())
    }

    /// Complete the handoff.
    pub fn complete(&mut self) -> Result<(), HandoffError> {
        if self.state != HandoffState::InProgress {
            return Err(HandoffError::InvalidState);
        }

        self.state = HandoffState::Completed;
        self.completed_handoffs += 1;
        Ok(())
    }

    /// Check for timeout and escalate if necessary.
    pub fn check_timeout(&mut self) -> HandoffState {
        if self.state != HandoffState::Initiated {
            return self.state;
        }

        if let Some(initiated) = self.initiated_at {
            let elapsed = initiated.elapsed();
            let stage_duration = self.config.acknowledgement_timeout.as_secs_f64()
                / self.config.escalation_stages as f64;

            let expected_stage = (elapsed.as_secs_f64() / stage_duration) as u32;
            if expected_stage > self.escalation_level
                && self.escalation_level < self.config.escalation_stages
            {
                self.escalation_level = expected_stage.min(self.config.escalation_stages);
            }

            if elapsed >= self.config.acknowledgement_timeout {
                self.state = HandoffState::TimedOut;
                self.failed_handoffs += 1;
            }
        }

        self.state
    }

    /// Abort the current handoff.
    pub fn abort(&mut self) {
        if self.state == HandoffState::Initiated || self.state == HandoffState::InProgress {
            self.state = HandoffState::Failed;
            self.failed_handoffs += 1;
        }
    }

    /// Reset to idle state.
    pub fn reset(&mut self) {
        self.state = HandoffState::Idle;
        self.direction = None;
        self.urgency = None;
        self.reason = None;
        self.initiated_at = None;
        self.escalation_level = 0;
    }
}

/// Handoff error types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffError {
    /// A handoff is already in progress.
    AlreadyInProgress,
    /// Invalid state for this operation.
    InvalidState,
    /// Vehicle speed is too high for handoff.
    SpeedTooHigh,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_idle() {
        let mgr = HandoffManager::with_defaults();
        assert_eq!(mgr.state(), HandoffState::Idle);
    }

    #[test]
    fn test_initiate_handoff() {
        let mut mgr = HandoffManager::with_defaults();
        let result = mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::ZoneBoundary,
            25.0,
        );
        assert!(result.is_ok());
        assert_eq!(mgr.state(), HandoffState::Initiated);
    }

    #[test]
    fn test_full_handoff_lifecycle() {
        let mut mgr = HandoffManager::with_defaults();
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::DriverRequested,
            25.0,
        )
        .unwrap();
        assert_eq!(mgr.state(), HandoffState::Initiated);

        mgr.acknowledge().unwrap();
        assert_eq!(mgr.state(), HandoffState::InProgress);

        mgr.complete().unwrap();
        assert_eq!(mgr.state(), HandoffState::Completed);
        assert_eq!(mgr.completed_count(), 1);
    }

    #[test]
    fn test_double_initiate_fails() {
        let mut mgr = HandoffManager::with_defaults();
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::SystemLimit,
            25.0,
        )
        .unwrap();
        let result = mgr.initiate(
            HandoffDirection::ToSystem,
            HandoffUrgency::Urgent,
            HandoffReason::Emergency,
            25.0,
        );
        assert_eq!(result.unwrap_err(), HandoffError::AlreadyInProgress);
    }

    #[test]
    fn test_acknowledge_wrong_state() {
        let mut mgr = HandoffManager::with_defaults();
        let result = mgr.acknowledge();
        assert_eq!(result.unwrap_err(), HandoffError::InvalidState);
    }

    #[test]
    fn test_complete_wrong_state() {
        let mut mgr = HandoffManager::with_defaults();
        let result = mgr.complete();
        assert_eq!(result.unwrap_err(), HandoffError::InvalidState);
    }

    #[test]
    fn test_high_speed_handoff_blocked() {
        let config = HandoffConfig {
            allow_high_speed_handoff: false,
            high_speed_threshold: 30.0,
            ..Default::default()
        };
        let mut mgr = HandoffManager::new(config);
        let result = mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::SystemLimit,
            35.0,
        );
        assert_eq!(result.unwrap_err(), HandoffError::SpeedTooHigh);
    }

    #[test]
    fn test_abort_increments_failed() {
        let mut mgr = HandoffManager::with_defaults();
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::SystemLimit,
            25.0,
        )
        .unwrap();
        mgr.abort();
        assert_eq!(mgr.state(), HandoffState::Failed);
        assert_eq!(mgr.failed_count(), 1);
    }

    #[test]
    fn test_reset_to_idle() {
        let mut mgr = HandoffManager::with_defaults();
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::SystemLimit,
            25.0,
        )
        .unwrap();
        mgr.reset();
        assert_eq!(mgr.state(), HandoffState::Idle);
    }

    #[test]
    fn test_timeout_detection() {
        let config = HandoffConfig {
            acknowledgement_timeout: Duration::from_millis(0), // Immediate timeout
            escalation_stages: 3,
            ..Default::default()
        };
        let mut mgr = HandoffManager::new(config);
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Urgent,
            HandoffReason::Emergency,
            25.0,
        )
        .unwrap();
        std::thread::sleep(Duration::from_millis(1));
        let state = mgr.check_timeout();
        assert_eq!(state, HandoffState::TimedOut);
        assert_eq!(mgr.failed_count(), 1);
    }

    #[test]
    fn test_can_reinitiate_after_completion() {
        let mut mgr = HandoffManager::with_defaults();
        mgr.initiate(
            HandoffDirection::ToHuman,
            HandoffUrgency::Planned,
            HandoffReason::DriverRequested,
            25.0,
        )
        .unwrap();
        mgr.acknowledge().unwrap();
        mgr.complete().unwrap();
        // Should be able to initiate again after completion
        let result = mgr.initiate(
            HandoffDirection::ToSystem,
            HandoffUrgency::Planned,
            HandoffReason::DriverRequested,
            25.0,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_urgency_ordering() {
        assert!(HandoffUrgency::Planned < HandoffUrgency::Prompt);
        assert!(HandoffUrgency::Prompt < HandoffUrgency::Urgent);
        assert!(HandoffUrgency::Urgent < HandoffUrgency::Emergency);
    }
}
