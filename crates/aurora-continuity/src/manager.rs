//! Continuity manager — automatic mode switching with graceful degradation.

use aurora_core::types::{ContinuityMode, IntegrityLevel};
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::health::{HealthStateMachine, Subsystem};

/// Records a mode transition for diagnostics.
#[derive(Debug, Clone)]
pub struct ModeTransition {
    pub from: ContinuityMode,
    pub to: ContinuityMode,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

/// Continuity manager that implements the No Hard Failure Policy.
///
/// Orchestrates automatic mode switching between Modes A-E,
/// manages recovery logic, and ensures service continuity.
pub struct ContinuityManager {
    health: HealthStateMachine,
    current_mode: ContinuityMode,
    previous_mode: Option<ContinuityMode>,
    transition_history: Vec<ModeTransition>,
    /// Time when current mode was entered.
    mode_entered_at: DateTime<Utc>,
    /// Maximum time in Mode D before switching to Mode E (seconds).
    max_mode_d_duration_s: f64,
    /// Whether recovery is pending (waiting for re-validation).
    recovery_pending: bool,
}

impl ContinuityManager {
    pub fn new() -> Self {
        Self {
            health: HealthStateMachine::new(),
            current_mode: ContinuityMode::ModeE,
            previous_mode: None,
            transition_history: Vec::new(),
            mode_entered_at: Utc::now(),
            max_mode_d_duration_s: 120.0, // 2 minutes in dead reckoning max
            recovery_pending: false,
        }
    }

    /// Update subsystem health and potentially trigger mode transition.
    pub fn update_health(&mut self, subsystem: Subsystem, healthy: bool, details: Option<String>) {
        if healthy {
            self.health.heartbeat(subsystem);
        } else {
            self.health
                .report_failure(subsystem, details.unwrap_or_default());
        }
        self.evaluate_mode();
    }

    /// Force-evaluate mode based on current health state.
    pub fn evaluate_mode(&mut self) {
        self.health.check_timeouts();

        let achievable = self.health.max_achievable_mode();
        let now = Utc::now();

        // Check Mode D timeout → force to Mode E.
        if self.current_mode == ContinuityMode::ModeD {
            let duration = (now - self.mode_entered_at).num_seconds() as f64;
            if duration > self.max_mode_d_duration_s {
                warn!(
                    duration_s = duration,
                    "Mode D timeout — switching to Mode E"
                );
                self.transition_to(ContinuityMode::ModeE, "Mode D timeout exceeded");
                return;
            }
        }

        // Degradation: only degrade, never upgrade without re-validation.
        match mode_rank(achievable).cmp(&mode_rank(self.current_mode)) {
            std::cmp::Ordering::Less => {
                // Degradation — apply immediately.
                let reason = format!(
                    "degradation: {} → {}",
                    self.current_mode, achievable
                );
                self.transition_to(achievable, &reason);
            }
            std::cmp::Ordering::Greater => {
                // Recovery — requires re-validation.
                if !self.recovery_pending {
                    info!(
                        from = %self.current_mode,
                        to = %achievable,
                        "recovery possible — pending re-validation"
                    );
                    self.recovery_pending = true;
                }
            }
            std::cmp::Ordering::Equal => {}
        }
    }

    /// Confirm recovery after re-validation (called by integrity engine).
    pub fn confirm_recovery(&mut self) {
        if self.recovery_pending {
            let achievable = self.health.max_achievable_mode();
            let reason = format!(
                "recovery validated: {} → {}",
                self.current_mode, achievable
            );
            self.transition_to(achievable, &reason);
            self.recovery_pending = false;
        }
    }

    /// Notify that integrity level has changed.
    pub fn integrity_changed(&mut self, level: IntegrityLevel) {
        match level {
            IntegrityLevel::Alert => {
                if self.current_mode != ContinuityMode::ModeE {
                    self.transition_to(
                        ContinuityMode::ModeE,
                        "integrity alert — emergency mode",
                    );
                }
            }
            IntegrityLevel::Warning => {
                if mode_rank(self.current_mode) > mode_rank(ContinuityMode::ModeC) {
                    self.transition_to(
                        ContinuityMode::ModeC,
                        "integrity warning — degrading to Mode C",
                    );
                }
            }
            _ => {}
        }
    }

    /// Current operating mode.
    pub fn current_mode(&self) -> ContinuityMode {
        self.current_mode
    }

    /// Whether recovery is pending re-validation.
    pub fn is_recovery_pending(&self) -> bool {
        self.recovery_pending
    }

    /// Get the transition history.
    pub fn transition_history(&self) -> &[ModeTransition] {
        &self.transition_history
    }

    /// Access the health state machine.
    pub fn health(&self) -> &HealthStateMachine {
        &self.health
    }

    /// Mutable access to the health state machine.
    pub fn health_mut(&mut self) -> &mut HealthStateMachine {
        &mut self.health
    }

    /// Time spent in current mode (seconds).
    pub fn time_in_current_mode_s(&self) -> f64 {
        (Utc::now() - self.mode_entered_at).num_milliseconds() as f64 / 1000.0
    }

    fn transition_to(&mut self, new_mode: ContinuityMode, reason: &str) {
        if new_mode == self.current_mode {
            return;
        }

        let transition = ModeTransition {
            from: self.current_mode,
            to: new_mode,
            reason: reason.to_string(),
            timestamp: Utc::now(),
        };

        info!(
            from = %self.current_mode,
            to = %new_mode,
            reason,
            "continuity mode transition"
        );

        self.previous_mode = Some(self.current_mode);
        self.current_mode = new_mode;
        self.mode_entered_at = Utc::now();
        self.transition_history.push(transition);
    }
}

impl Default for ContinuityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Numeric rank for modes (higher = better).
fn mode_rank(mode: ContinuityMode) -> u8 {
    match mode {
        ContinuityMode::ModeA => 5,
        ContinuityMode::ModeB => 4,
        ContinuityMode::ModeC => 3,
        ContinuityMode::ModeD => 2,
        ContinuityMode::ModeE => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_mode_e() {
        let cm = ContinuityManager::new();
        assert_eq!(cm.current_mode(), ContinuityMode::ModeE);
    }

    #[test]
    fn gnss_heartbeat_triggers_recovery_pending() {
        let mut cm = ContinuityManager::new();
        cm.update_health(Subsystem::GnssReceiver, true, None);
        // Should be pending recovery (need re-validation before upgrade).
        assert!(cm.is_recovery_pending());
    }

    #[test]
    fn recovery_confirmation_upgrades_mode() {
        let mut cm = ContinuityManager::new();
        cm.update_health(Subsystem::GnssReceiver, true, None);
        cm.update_health(Subsystem::CorrectionEngine, true, None);
        assert!(cm.is_recovery_pending());

        cm.confirm_recovery();
        assert_eq!(cm.current_mode(), ContinuityMode::ModeA);
        assert!(!cm.is_recovery_pending());
    }

    #[test]
    fn integrity_alert_forces_mode_e() {
        let mut cm = ContinuityManager::new();
        cm.update_health(Subsystem::GnssReceiver, true, None);
        cm.confirm_recovery();

        cm.integrity_changed(IntegrityLevel::Alert);
        assert_eq!(cm.current_mode(), ContinuityMode::ModeE);
    }

    #[test]
    fn transition_history_is_recorded() {
        let mut cm = ContinuityManager::new();
        cm.update_health(Subsystem::GnssReceiver, true, None);
        cm.confirm_recovery();

        assert!(!cm.transition_history().is_empty());
    }
}
