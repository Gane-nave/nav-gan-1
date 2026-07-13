//! Break recommendation engine — suggests optimal rest stops based on fatigue and route context.

use std::time::{Duration, Instant};

/// Break urgency level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BreakUrgency {
    /// No break needed.
    None,
    /// Optional — a break would be nice.
    Optional,
    /// Recommended — driver should take a break soon.
    Recommended,
    /// Urgent — driver must take a break.
    Urgent,
    /// Mandatory — safety regulations require a break.
    Mandatory,
}

/// A recommended break.
#[derive(Debug, Clone)]
pub struct BreakRecommendation {
    /// Urgency of the break.
    pub urgency: BreakUrgency,
    /// Recommended duration of the break.
    pub recommended_duration: Duration,
    /// Reason for the recommendation.
    pub reason: BreakReason,
    /// Maximum distance to continue before break (km).
    pub max_continue_km: f64,
}

/// Reason for a break recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakReason {
    /// Fatigue detected.
    Fatigue,
    /// Long continuous driving time.
    DrivingDuration,
    /// Regulatory requirement (e.g., EU driving hours).
    Regulatory,
    /// Scheduled break time.
    Scheduled,
    /// Health metric threshold.
    Health,
}

/// Configuration for break recommendations.
#[derive(Debug, Clone)]
pub struct BreakConfig {
    /// Maximum continuous driving before recommending a break.
    pub max_continuous_driving: Duration,
    /// Maximum driving before mandatory break (regulatory).
    pub max_regulatory_driving: Duration,
    /// Minimum break duration for short breaks.
    pub min_short_break: Duration,
    /// Minimum break duration for mandatory breaks.
    pub min_mandatory_break: Duration,
    /// Interval for scheduled break reminders.
    pub scheduled_interval: Duration,
}

impl Default for BreakConfig {
    fn default() -> Self {
        Self {
            max_continuous_driving: Duration::from_secs(7200), // 2 hours
            max_regulatory_driving: Duration::from_secs(16200), // 4.5 hours (EU)
            min_short_break: Duration::from_secs(900),         // 15 minutes
            min_mandatory_break: Duration::from_secs(2700),    // 45 minutes
            scheduled_interval: Duration::from_secs(7200),     // 2 hours
        }
    }
}

/// Break recommendation engine.
pub struct BreakAdvisor {
    config: BreakConfig,
    driving_start: Option<Instant>,
    last_break: Option<Instant>,
    breaks_taken: u32,
    total_break_duration: Duration,
}

impl BreakAdvisor {
    /// Create a new break advisor.
    pub fn new(config: BreakConfig) -> Self {
        Self {
            config,
            driving_start: None,
            last_break: None,
            breaks_taken: 0,
            total_break_duration: Duration::ZERO,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(BreakConfig::default())
    }

    /// Start a driving session.
    pub fn start_session(&mut self) {
        let now = Instant::now();
        self.driving_start = Some(now);
        self.last_break = Some(now);
        self.breaks_taken = 0;
        self.total_break_duration = Duration::ZERO;
    }

    /// Record that the driver took a break.
    pub fn record_break(&mut self, duration: Duration) {
        self.last_break = Some(Instant::now());
        self.breaks_taken += 1;
        self.total_break_duration += duration;
    }

    /// Get the number of breaks taken.
    pub fn breaks_taken(&self) -> u32 {
        self.breaks_taken
    }

    /// Get recommendation based on current driving state.
    pub fn recommend(&self, driving_duration: Duration, fatigue_score: f64) -> BreakRecommendation {
        // Check mandatory regulatory break
        if driving_duration >= self.config.max_regulatory_driving {
            return BreakRecommendation {
                urgency: BreakUrgency::Mandatory,
                recommended_duration: self.config.min_mandatory_break,
                reason: BreakReason::Regulatory,
                max_continue_km: 0.0,
            };
        }

        // Check fatigue-based break
        if fatigue_score < 0.3 {
            return BreakRecommendation {
                urgency: BreakUrgency::Urgent,
                recommended_duration: self.config.min_short_break,
                reason: BreakReason::Fatigue,
                max_continue_km: 5.0,
            };
        }

        if fatigue_score < 0.5 {
            return BreakRecommendation {
                urgency: BreakUrgency::Recommended,
                recommended_duration: self.config.min_short_break,
                reason: BreakReason::Fatigue,
                max_continue_km: 20.0,
            };
        }

        // Check continuous driving time
        if driving_duration >= self.config.max_continuous_driving {
            return BreakRecommendation {
                urgency: BreakUrgency::Recommended,
                recommended_duration: self.config.min_short_break,
                reason: BreakReason::DrivingDuration,
                max_continue_km: 30.0,
            };
        }

        // Approaching continuous driving limit
        let limit_80 = self.config.max_continuous_driving.mul_f64(0.8);
        if driving_duration >= limit_80 {
            return BreakRecommendation {
                urgency: BreakUrgency::Optional,
                recommended_duration: self.config.min_short_break,
                reason: BreakReason::DrivingDuration,
                max_continue_km: 50.0,
            };
        }

        BreakRecommendation {
            urgency: BreakUrgency::None,
            recommended_duration: Duration::ZERO,
            reason: BreakReason::Scheduled,
            max_continue_km: f64::MAX,
        }
    }

    /// Time since last break (or session start).
    pub fn time_since_last_break(&self) -> Option<Duration> {
        self.last_break.map(|t| t.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_break_needed_initially() {
        let advisor = BreakAdvisor::with_defaults();
        let rec = advisor.recommend(Duration::from_secs(1800), 0.9); // 30 min, alert
        assert_eq!(rec.urgency, BreakUrgency::None);
    }

    #[test]
    fn test_optional_break_approaching_limit() {
        let advisor = BreakAdvisor::with_defaults();
        // 80% of 2-hour limit = 96 min
        let rec = advisor.recommend(Duration::from_secs(6000), 0.9);
        assert_eq!(rec.urgency, BreakUrgency::Optional);
        assert_eq!(rec.reason, BreakReason::DrivingDuration);
    }

    #[test]
    fn test_recommended_break_at_limit() {
        let advisor = BreakAdvisor::with_defaults();
        let rec = advisor.recommend(Duration::from_secs(7200), 0.9); // Exactly 2 hours
        assert_eq!(rec.urgency, BreakUrgency::Recommended);
        assert_eq!(rec.reason, BreakReason::DrivingDuration);
    }

    #[test]
    fn test_mandatory_break_regulatory() {
        let advisor = BreakAdvisor::with_defaults();
        let rec = advisor.recommend(Duration::from_secs(16200), 0.9); // 4.5 hours
        assert_eq!(rec.urgency, BreakUrgency::Mandatory);
        assert_eq!(rec.reason, BreakReason::Regulatory);
        assert_eq!(rec.max_continue_km, 0.0);
    }

    #[test]
    fn test_urgent_break_on_high_fatigue() {
        let advisor = BreakAdvisor::with_defaults();
        let rec = advisor.recommend(Duration::from_secs(1800), 0.2); // 30 min but very fatigued
        assert_eq!(rec.urgency, BreakUrgency::Urgent);
        assert_eq!(rec.reason, BreakReason::Fatigue);
        assert!(rec.max_continue_km <= 5.0);
    }

    #[test]
    fn test_recommended_break_moderate_fatigue() {
        let advisor = BreakAdvisor::with_defaults();
        let rec = advisor.recommend(Duration::from_secs(1800), 0.4); // Moderate fatigue
        assert_eq!(rec.urgency, BreakUrgency::Recommended);
        assert_eq!(rec.reason, BreakReason::Fatigue);
    }

    #[test]
    fn test_fatigue_overrides_duration() {
        let advisor = BreakAdvisor::with_defaults();
        // Short drive but critical fatigue
        let rec = advisor.recommend(Duration::from_secs(600), 0.1);
        assert_eq!(rec.urgency, BreakUrgency::Urgent);
        assert_eq!(rec.reason, BreakReason::Fatigue);
    }

    #[test]
    fn test_regulatory_overrides_fatigue() {
        let advisor = BreakAdvisor::with_defaults();
        // Over regulatory limit, even if alert
        let rec = advisor.recommend(Duration::from_secs(20000), 0.9);
        assert_eq!(rec.urgency, BreakUrgency::Mandatory);
        assert_eq!(rec.reason, BreakReason::Regulatory);
    }

    #[test]
    fn test_break_tracking() {
        let mut advisor = BreakAdvisor::with_defaults();
        advisor.start_session();
        assert_eq!(advisor.breaks_taken(), 0);
        advisor.record_break(Duration::from_secs(900));
        assert_eq!(advisor.breaks_taken(), 1);
        advisor.record_break(Duration::from_secs(600));
        assert_eq!(advisor.breaks_taken(), 2);
    }

    #[test]
    fn test_urgency_ordering() {
        assert!(BreakUrgency::None < BreakUrgency::Optional);
        assert!(BreakUrgency::Optional < BreakUrgency::Recommended);
        assert!(BreakUrgency::Recommended < BreakUrgency::Urgent);
        assert!(BreakUrgency::Urgent < BreakUrgency::Mandatory);
    }
}
